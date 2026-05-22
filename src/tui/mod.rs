pub mod animation;
pub mod ascii_font;
pub mod events;
pub mod mouse;
pub mod renderer;
pub mod theme;
pub mod widgets;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use crate::{
    config::Config,
    domain::stats::StatsSummary,
    domain::{note::Note, task::Task, timer::TimerSnapshot},
};

use theme::Theme;

pub struct TerminalSession {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    mouse: bool,
}

impl TerminalSession {
    pub fn enter(mouse: bool) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        if mouse {
            execute!(stdout, EnableMouseCapture)?;
        }
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal, mouse })
    }

    pub fn draw(&mut self, state: &mut UiState, snapshot: &TimerSnapshot) -> Result<()> {
        self.terminal
            .draw(|frame| renderer::render(frame, state, snapshot))?;
        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        if self.mouse {
            let _ = execute!(self.terminal.backend_mut(), DisableMouseCapture);
        }
        let _ = self.terminal.show_cursor();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Timer,
    Tasks,
    Notes,
    Stats,
    Event,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Task,
    Note,
    EditNote(u64),
    Event,
}

pub struct UiState {
    config: Config,
    task: Option<String>,
    tasks: Vec<Task>,
    notes: Vec<Note>,
    stats: StatsSummary,
    event_status: Option<String>,
    theme_index: usize,
    font_index: usize,
    frame: u64,
    show_help: bool,
    tab: Tab,
    input_mode: Option<InputMode>,
    input_buffer: String,
    mouse_zones: mouse::MouseZones,
}

impl UiState {
    pub fn new(
        config: Config,
        task: Option<String>,
        tasks: Vec<Task>,
        notes: Vec<Note>,
        stats: StatsSummary,
    ) -> Self {
        let themes = Theme::catalog();
        let theme_index = themes
            .iter()
            .position(|theme| theme.name == config.visuals.theme)
            .unwrap_or_default();
        let font_index = ascii_font::FONTS
            .iter()
            .position(|font| *font == config.visuals.font)
            .unwrap_or_default();
        Self {
            config,
            task,
            tasks,
            notes,
            stats,
            event_status: None,
            theme_index,
            font_index,
            frame: 0,
            show_help: false,
            tab: Tab::Timer,
            input_mode: None,
            input_buffer: String::new(),
            mouse_zones: mouse::MouseZones::default(),
        }
    }

    pub fn tick_rate(&self) -> Duration {
        if self.config.visuals.animations {
            Duration::from_millis(match self.config.visuals.animation_speed.as_str() {
                "slow" => 500,
                "fast" => 120,
                _ => 250,
            })
        } else {
            Duration::from_millis(750)
        }
    }

    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    pub fn theme(&self) -> Theme {
        Theme::catalog()[self.theme_index].clone()
    }

    pub fn theme_name(&self) -> String {
        Theme::catalog()[self.theme_index].name.clone()
    }

    pub fn font(&self) -> &str {
        ascii_font::FONTS[self.font_index]
    }

    pub fn task(&self) -> &str {
        self.task.as_deref().unwrap_or("No task selected")
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn stats(&self) -> &StatsSummary {
        &self.stats
    }

    pub fn event_status(&self) -> &str {
        self.event_status
            .as_deref()
            .unwrap_or("No event countdown configured")
    }

    pub fn input_mode(&self) -> Option<&InputMode> {
        self.input_mode.as_ref()
    }

    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn tab(&self) -> Tab {
        self.tab
    }

    pub fn mouse_zones_mut(&mut self) -> &mut mouse::MouseZones {
        &mut self.mouse_zones
    }

    pub fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % Theme::catalog().len();
    }

    pub fn next_font(&mut self) {
        self.font_index = (self.font_index + 1) % ascii_font::FONTS.len();
    }

    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Timer => Tab::Tasks,
            Tab::Tasks => Tab::Notes,
            Tab::Notes => Tab::Stats,
            Tab::Stats => Tab::Event,
            Tab::Event => Tab::Settings,
            Tab::Settings => Tab::Timer,
        };
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn mouse_action(&self, x: u16, y: u16) -> Option<mouse::MouseAction> {
        self.mouse_zones.hit(x, y)
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }

    pub fn open_task_input(&mut self) {
        self.input_mode = Some(InputMode::Task);
        self.input_buffer.clear();
    }

    pub fn open_note_input(&mut self) {
        self.input_mode = Some(InputMode::Note);
        self.input_buffer.clear();
    }

    pub fn open_event_input(&mut self) {
        self.input_mode = Some(InputMode::Event);
        self.input_buffer.clear();
    }

    pub fn open_edit_latest_note(&mut self) {
        if let Some(note) = self.notes.first() {
            self.input_mode = Some(InputMode::EditNote(note.id));
            self.input_buffer = note.body.clone();
        }
    }

    pub fn push_input(&mut self, ch: char) {
        if self.input_mode.is_some() && !ch.is_control() {
            self.input_buffer.push(ch);
        }
    }

    pub fn pop_input(&mut self) {
        self.input_buffer.pop();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = None;
        self.input_buffer.clear();
    }

    pub fn take_input(&mut self) -> Option<(InputMode, String)> {
        let mode = self.input_mode.take()?;
        let value = self.input_buffer.trim().to_string();
        self.input_buffer.clear();
        Some((mode, value))
    }

    pub fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    pub fn set_notes(&mut self, notes: Vec<Note>) {
        self.notes = notes;
    }

    pub fn set_stats(&mut self, stats: StatsSummary) {
        self.stats = stats;
    }

    pub fn set_event_status(&mut self, event_status: String) {
        self.event_status = Some(event_status);
    }

    pub fn set_task(&mut self, task: Option<String>) {
        self.task = task;
    }
}
