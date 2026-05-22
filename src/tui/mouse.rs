use ratatui::layout::Rect;

use super::Tab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    Start,
    Pause,
    Reset,
    Skip,
    Theme,
    Help,
    Tab(Tab),
    AddTask,
    CompleteTask,
    DeleteTask,
    AddNote,
    CompleteNote,
    EditNote,
    DeleteNote,
    AddEvent,
    SelectTask(usize),
}

#[derive(Debug, Clone, Default)]
pub struct MouseZones {
    zones: Vec<(Rect, MouseAction)>,
}

impl MouseZones {
    pub fn clear(&mut self) {
        self.zones.clear();
    }

    pub fn add(&mut self, rect: Rect, action: MouseAction) {
        self.zones.push((rect, action));
    }

    pub fn hit(&self, x: u16, y: u16) -> Option<MouseAction> {
        self.zones
            .iter()
            .find(|(rect, _)| {
                x >= rect.x
                    && x < rect.x.saturating_add(rect.width)
                    && y >= rect.y
                    && y < rect.y.saturating_add(rect.height)
            })
            .map(|(_, action)| *action)
    }
}
