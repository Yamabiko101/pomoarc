use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Start,
    PauseResume,
    Reset,
    Skip,
    Quit,
    Help,
    Theme,
    Font,
    Mode,
    AddMinute,
    RemoveMinute,
    Tab,
    Mouse(u16, u16),
    None,
}

pub fn poll(timeout: Duration) -> Result<bool> {
    Ok(event::poll(timeout)?)
}

pub fn read() -> Result<InputEvent> {
    Ok(match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('s') => InputEvent::Start,
            KeyCode::Char(' ') => InputEvent::PauseResume,
            KeyCode::Char('r') => InputEvent::Reset,
            KeyCode::Char('n') => InputEvent::Skip,
            KeyCode::Char('q') | KeyCode::Esc => InputEvent::Quit,
            KeyCode::Char('?') => InputEvent::Help,
            KeyCode::Char('t') => InputEvent::Theme,
            KeyCode::Char('a') => InputEvent::Font,
            KeyCode::Char('m') => InputEvent::Mode,
            KeyCode::Char('+') => InputEvent::AddMinute,
            KeyCode::Char('-') => InputEvent::RemoveMinute,
            KeyCode::Tab => InputEvent::Tab,
            _ => InputEvent::None,
        },
        Event::Mouse(mouse) => match mouse.kind {
            MouseEventKind::Down(_) => InputEvent::Mouse(mouse.column, mouse.row),
            _ => InputEvent::None,
        },
        _ => InputEvent::None,
    })
}
