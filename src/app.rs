use anyhow::Result;
use chrono::Local;
use std::time::{Duration, Instant};

use crate::{
    cli::{NoteCommand, TaskCommand},
    config::Config,
    domain::{
        session::SessionRecord,
        task::Task,
        timer::{TimerEngine, TimerMode},
    },
    notifications,
    storage::JsonlStore,
    tui,
    tui::mouse::MouseAction,
};

pub fn run_tui(config: Config, profile: Option<String>, task: Option<String>) -> Result<()> {
    let profile_name = profile.unwrap_or_else(|| "default".to_string());
    let timer_profile = config.profile(Some(&profile_name));
    let engine = TimerEngine::pomodoro(timer_profile);
    run_engine(config, engine, Some(profile_name), task)
}

pub fn run_timer_view(config: Config, mode: TimerMode, task: Option<String>) -> Result<()> {
    run_engine(config, TimerEngine::new(mode), None, task)
}

pub fn handle_task(command: TaskCommand) -> Result<()> {
    let store = JsonlStore::new(Config::data_dir()?.join("sessions.jsonl"));
    match command {
        TaskCommand::Add { title, tag } => {
            store.add_task(&Task::new(title.clone(), tag))?;
            println!("Task added: {title}");
        }
        TaskCommand::List => {
            let tasks = store.tasks()?;
            if tasks.is_empty() {
                println!("No tasks yet.");
            } else {
                for task in tasks {
                    println!(
                        "[{}] {} ({})",
                        if task.done { "x" } else { " " },
                        task.title,
                        task.tag
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn handle_note(command: NoteCommand) -> Result<()> {
    let store = JsonlStore::new(Config::data_dir()?.join("sessions.jsonl"));
    match command {
        NoteCommand::Add { body } => {
            let note = store.add_note(body)?;
            println!("Note #{} added.", note.id);
        }
        NoteCommand::List => {
            let notes = store.notes()?;
            if notes.is_empty() {
                println!("No notes yet.");
            } else {
                for note in notes {
                    println!("#{} {}", note.id, note.body);
                }
            }
        }
        NoteCommand::Edit { id, body } => match store.update_note(id, body)? {
            Some(note) => println!("Note #{} updated.", note.id),
            None => println!("Note #{id} not found."),
        },
        NoteCommand::Delete { id } => {
            if store.delete_note(id)? {
                println!("Note #{id} deleted.");
            } else {
                println!("Note #{id} not found.");
            }
        }
    }
    Ok(())
}

fn run_engine(
    config: Config,
    mut engine: TimerEngine,
    profile_name: Option<String>,
    task: Option<String>,
) -> Result<()> {
    let store = JsonlStore::new(Config::data_dir()?.join("sessions.jsonl"));
    let tasks = store.tasks()?;
    let notes = latest_notes(&store)?;
    let mut terminal = tui::TerminalSession::enter(config.input.mouse)?;
    let mut state = tui::UiState::new(config.clone(), task.clone(), tasks, notes);
    let session_started_at = Local::now();
    let mut last_tick = Instant::now();

    loop {
        let snapshot = engine.snapshot();
        terminal.draw(&mut state, &snapshot)?;

        if tui::events::poll(Duration::from_millis(80))? {
            let event = tui::events::read()?;
            if state.input_mode().is_some() {
                match event {
                    tui::events::InputEvent::Input(ch) => state.push_input(ch),
                    tui::events::InputEvent::Backspace => state.pop_input(),
                    tui::events::InputEvent::Cancel => state.cancel_input(),
                    tui::events::InputEvent::Enter => {
                        if let Some((mode, value)) = state.take_input() {
                            if !value.is_empty() {
                                match mode {
                                    tui::InputMode::Task => {
                                        store.add_task(&Task::new(value.clone(), None))?;
                                        state.set_tasks(store.tasks()?);
                                        state.set_task(Some(value));
                                    }
                                    tui::InputMode::Note => {
                                        store.add_note(value)?;
                                        state.set_notes(latest_notes(&store)?);
                                    }
                                    tui::InputMode::EditNote(id) => {
                                        let _ = store.update_note(id, value)?;
                                        state.set_notes(latest_notes(&store)?);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                continue;
            }

            match event {
                tui::events::InputEvent::AddMinute => engine.add_minute(),
                tui::events::InputEvent::RemoveMinute => engine.remove_minute(),
                tui::events::InputEvent::Help => state.toggle_help(),
                tui::events::InputEvent::Tab => state.next_tab(),
                tui::events::InputEvent::Enter => match state.tab() {
                    tui::Tab::Tasks => {
                        if let Some(task) = state.tasks().first() {
                            state.set_task(Some(task.title.clone()));
                        }
                    }
                    tui::Tab::Notes => state.open_note_input(),
                    _ => {}
                },
                tui::events::InputEvent::Input('s') => engine.start(),
                tui::events::InputEvent::Input(' ') => engine.toggle(),
                tui::events::InputEvent::Input('r') => engine.reset(),
                tui::events::InputEvent::Input('n') => engine.skip(),
                tui::events::InputEvent::Input('q') => break,
                tui::events::InputEvent::Input('t') => state.next_theme(),
                tui::events::InputEvent::Input('a') => state.next_font(),
                tui::events::InputEvent::Input('m') => state.next_tab(),
                tui::events::InputEvent::Input('i') => match state.tab() {
                    tui::Tab::Tasks => state.open_task_input(),
                    tui::Tab::Notes => state.open_note_input(),
                    _ => {}
                },
                tui::events::InputEvent::Input('e') if state.tab() == tui::Tab::Notes => {
                    state.open_edit_latest_note();
                }
                tui::events::InputEvent::Input('x') if state.tab() == tui::Tab::Notes => {
                    if let Some(note) = state.notes().first() {
                        let _ = store.delete_note(note.id)?;
                        state.set_notes(latest_notes(&store)?);
                    }
                }
                tui::events::InputEvent::Mouse(x, y) => match state.mouse_action(x, y) {
                    Some(MouseAction::Start) => engine.start(),
                    Some(MouseAction::Pause) => engine.toggle(),
                    Some(MouseAction::Reset) => engine.reset(),
                    Some(MouseAction::Skip) => engine.skip(),
                    Some(MouseAction::Theme) => state.next_theme(),
                    Some(MouseAction::Help) => state.toggle_help(),
                    Some(MouseAction::Tab(tab)) => state.set_tab(tab),
                    Some(MouseAction::AddTask) => state.open_task_input(),
                    Some(MouseAction::AddNote) => state.open_note_input(),
                    Some(MouseAction::EditNote) => state.open_edit_latest_note(),
                    Some(MouseAction::DeleteNote) => {
                        if let Some(note) = state.notes().first() {
                            let _ = store.delete_note(note.id)?;
                            state.set_notes(latest_notes(&store)?);
                        }
                    }
                    Some(MouseAction::SelectTask(index)) => {
                        if let Some(task) = state.tasks().get(index) {
                            state.set_task(Some(task.title.clone()));
                        }
                    }
                    None => {}
                },
                tui::events::InputEvent::Cancel => break,
                tui::events::InputEvent::Backspace | tui::events::InputEvent::Input(_) => {}
                tui::events::InputEvent::None => {}
            }
        }

        if last_tick.elapsed() >= state.tick_rate() {
            last_tick = Instant::now();
            state.tick();
            let finished = engine.tick();
            if finished {
                let ended_at = Local::now();
                let snap = engine.snapshot();
                store.append_session(&SessionRecord {
                    started_at: session_started_at,
                    ended_at,
                    planned_seconds: snap.total.as_secs(),
                    actual_seconds: snap.elapsed.as_secs().min(snap.total.as_secs()),
                    session_type: snap.phase.label().to_string(),
                    task: task.clone(),
                    tags: task
                        .as_ref()
                        .map(|_| vec!["custom".to_string()])
                        .unwrap_or_default(),
                    completed: true,
                    profile: profile_name.clone().unwrap_or_else(|| "ad-hoc".to_string()),
                    theme: Some(state.theme_name()),
                    energy: None,
                    mood: None,
                    intention: None,
                    note: None,
                })?;
                if config.notifications.enabled {
                    let _ = notifications::notify("pomoarc", "Session complete.");
                }
                if config.notifications.sound {
                    let _ = notifications::play_sound();
                }
            }
        }
    }

    Ok(())
}

fn latest_notes(store: &JsonlStore) -> Result<Vec<crate::domain::note::Note>> {
    let mut notes = store.notes()?;
    notes.sort_by_key(|note| std::cmp::Reverse(note.updated_at));
    notes.truncate(6);
    Ok(notes)
}
