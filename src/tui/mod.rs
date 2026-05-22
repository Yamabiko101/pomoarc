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

use crate::{config::Config, domain::timer::TimerSnapshot};

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
    Stats,
    Tasks,
    Settings,
}

pub struct UiState {
    config: Config,
    task: Option<String>,
    theme_index: usize,
    font_index: usize,
    frame: u64,
    show_help: bool,
    tab: Tab,
    mouse_zones: mouse::MouseZones,
}

impl UiState {
    pub fn new(config: Config, task: Option<String>) -> Self {
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
            theme_index,
            font_index,
            frame: 0,
            show_help: false,
            tab: Tab::Timer,
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
            Tab::Timer => Tab::Stats,
            Tab::Stats => Tab::Tasks,
            Tab::Tasks => Tab::Settings,
            Tab::Settings => Tab::Timer,
        };
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn handle_mouse(&mut self, x: u16, y: u16, engine: &mut crate::domain::timer::TimerEngine) {
        match self.mouse_zones.hit(x, y) {
            Some(mouse::MouseAction::Start) => engine.start(),
            Some(mouse::MouseAction::Pause) => engine.toggle(),
            Some(mouse::MouseAction::Reset) => engine.reset(),
            Some(mouse::MouseAction::Skip) => engine.skip(),
            Some(mouse::MouseAction::Theme) => self.next_theme(),
            Some(mouse::MouseAction::Help) => self.toggle_help(),
            Some(mouse::MouseAction::Tab(tab)) => self.tab = tab,
            None => {}
        }
    }
}
