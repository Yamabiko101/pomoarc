use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Help,
    AddMinute,
    RemoveMinute,
    Tab,
    Enter,
    Backspace,
    Cancel,
    Input(char),
    Mouse(u16, u16),
    None,
}

pub fn poll(timeout: Duration) -> Result<bool> {
    Ok(event::poll(timeout)?)
}

pub fn read() -> Result<InputEvent> {
    Ok(match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Esc => InputEvent::Cancel,
            KeyCode::Char('?') => InputEvent::Help,
            KeyCode::Char('+') => InputEvent::AddMinute,
            KeyCode::Char('-') => InputEvent::RemoveMinute,
            KeyCode::Enter => InputEvent::Enter,
            KeyCode::Backspace => InputEvent::Backspace,
            KeyCode::Tab => InputEvent::Tab,
            KeyCode::Char(ch) => InputEvent::Input(ch),
            _ => InputEvent::None,
        },
        Event::Mouse(mouse) => match mouse.kind {
            MouseEventKind::Down(_) => InputEvent::Mouse(mouse.column, mouse.row),
            _ => InputEvent::None,
        },
        _ => InputEvent::None,
    })
}
