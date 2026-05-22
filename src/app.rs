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
    let notes = store
        .notes()?
        .into_iter()
        .rev()
        .take(5)
        .map(|note| format!("#{} {}", note.id, note.body))
        .collect();
    let mut terminal = tui::TerminalSession::enter(config.input.mouse)?;
    let mut state = tui::UiState::new(config.clone(), task.clone(), notes);
    let session_started_at = Local::now();
    let mut last_tick = Instant::now();

    loop {
        let snapshot = engine.snapshot();
        terminal.draw(&mut state, &snapshot)?;

        if tui::events::poll(Duration::from_millis(80))? {
            match tui::events::read()? {
                tui::events::InputEvent::Start => engine.start(),
                tui::events::InputEvent::PauseResume => engine.toggle(),
                tui::events::InputEvent::Reset => engine.reset(),
                tui::events::InputEvent::Skip => engine.skip(),
                tui::events::InputEvent::AddMinute => engine.add_minute(),
                tui::events::InputEvent::RemoveMinute => engine.remove_minute(),
                tui::events::InputEvent::Theme => state.next_theme(),
                tui::events::InputEvent::Font => state.next_font(),
                tui::events::InputEvent::Mode => state.next_tab(),
                tui::events::InputEvent::Help => state.toggle_help(),
                tui::events::InputEvent::Tab => state.next_tab(),
                tui::events::InputEvent::Mouse(x, y) => state.handle_mouse(x, y, &mut engine),
                tui::events::InputEvent::Quit => break,
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
