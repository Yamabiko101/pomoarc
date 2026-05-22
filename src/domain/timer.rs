use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::profile::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Focus,
    ShortBreak,
    LongBreak,
    Paused,
    Finished,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Focus => "Focus",
            Self::ShortBreak => "Short break",
            Self::LongBreak => "Long break",
            Self::Paused => "Paused",
            Self::Finished => "Finished",
        }
    }
}

#[derive(Debug, Clone)]
pub enum TimerMode {
    Pomodoro { profile: Profile },
    Countdown { seconds: u64 },
    Stopwatch,
    EventCountdown { name: String, at: DateTime<Local> },
}

#[derive(Debug, Clone)]
pub struct TimerSnapshot {
    pub phase: Phase,
    pub remaining: Duration,
    pub elapsed: Duration,
    pub total: Duration,
    pub cycle: u32,
    pub cycle_target: u32,
    pub running: bool,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct TimerEngine {
    mode: TimerMode,
    phase: Phase,
    cycle: u32,
    started_at: Option<Instant>,
    accumulated: Duration,
    current_total: Duration,
    previous_phase: Phase,
}

impl TimerEngine {
    pub fn pomodoro(profile: Profile) -> Self {
        let total = Duration::from_secs(profile.focus_minutes * 60);
        Self {
            mode: TimerMode::Pomodoro { profile },
            phase: Phase::Focus,
            cycle: 1,
            started_at: None,
            accumulated: Duration::ZERO,
            current_total: total,
            previous_phase: Phase::Focus,
        }
    }

    pub fn new(mode: TimerMode) -> Self {
        match mode.clone() {
            TimerMode::Pomodoro { profile } => Self::pomodoro(profile),
            TimerMode::Countdown { seconds } => Self {
                mode,
                phase: Phase::Focus,
                cycle: 1,
                started_at: None,
                accumulated: Duration::ZERO,
                current_total: Duration::from_secs(seconds),
                previous_phase: Phase::Focus,
            },
            TimerMode::Stopwatch => Self {
                mode,
                phase: Phase::Focus,
                cycle: 1,
                started_at: None,
                accumulated: Duration::ZERO,
                current_total: Duration::from_secs(24 * 60 * 60),
                previous_phase: Phase::Focus,
            },
            TimerMode::EventCountdown { at, .. } => {
                let seconds = (at - Local::now()).num_seconds().max(0) as u64;
                Self {
                    mode,
                    phase: Phase::Focus,
                    cycle: 1,
                    started_at: None,
                    accumulated: Duration::ZERO,
                    current_total: Duration::from_secs(seconds),
                    previous_phase: Phase::Focus,
                }
            }
        }
    }

    pub fn start(&mut self) {
        if self.phase == Phase::Finished {
            self.reset();
        }
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
            if self.phase == Phase::Paused {
                self.phase = self.previous_phase;
            }
        }
    }

    pub fn pause(&mut self) {
        if let Some(started_at) = self.started_at.take() {
            self.accumulated += started_at.elapsed();
            self.previous_phase = self.phase;
            self.phase = Phase::Paused;
        }
    }

    pub fn toggle(&mut self) {
        if self.started_at.is_some() {
            self.pause();
        } else {
            self.start();
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::Focus;
        self.cycle = 1;
        self.started_at = None;
        self.accumulated = Duration::ZERO;
        self.current_total = self.initial_total();
        self.previous_phase = Phase::Focus;
    }

    pub fn add_minute(&mut self) {
        self.current_total += Duration::from_secs(60);
    }

    pub fn remove_minute(&mut self) {
        self.current_total = self.current_total.saturating_sub(Duration::from_secs(60));
    }

    pub fn skip(&mut self) {
        match self.mode.clone() {
            TimerMode::Pomodoro { profile } => self.advance_pomodoro(&profile),
            TimerMode::Stopwatch
            | TimerMode::Countdown { .. }
            | TimerMode::EventCountdown { .. } => {
                self.phase = Phase::Finished;
                self.started_at = None;
            }
        }
    }

    pub fn tick(&mut self) -> bool {
        if matches!(self.mode, TimerMode::EventCountdown { .. }) {
            self.current_total = Duration::from_secs(self.event_remaining_seconds());
        }

        if self.started_at.is_some()
            && !matches!(self.mode, TimerMode::Stopwatch)
            && self.elapsed() >= self.current_total
        {
            self.skip();
            return true;
        }
        false
    }

    pub fn snapshot(&self) -> TimerSnapshot {
        let elapsed = self.elapsed();
        let remaining = match self.mode {
            TimerMode::Stopwatch => elapsed,
            _ => self.current_total.saturating_sub(elapsed),
        };
        TimerSnapshot {
            phase: self.phase,
            remaining,
            elapsed,
            total: self.current_total,
            cycle: self.cycle,
            cycle_target: self.cycle_target(),
            running: self.started_at.is_some(),
            label: self.label(),
        }
    }

    fn elapsed(&self) -> Duration {
        self.accumulated
            + self
                .started_at
                .map_or(Duration::ZERO, |started_at| started_at.elapsed())
    }

    fn advance_pomodoro(&mut self, profile: &Profile) {
        self.started_at = None;
        self.accumulated = Duration::ZERO;
        self.phase = match self.phase {
            Phase::Focus if self.cycle.is_multiple_of(profile.long_break_every) => Phase::LongBreak,
            Phase::Focus => Phase::ShortBreak,
            Phase::ShortBreak | Phase::LongBreak | Phase::Paused | Phase::Finished => {
                self.cycle += 1;
                Phase::Focus
            }
        };
        self.previous_phase = self.phase;
        self.current_total = Duration::from_secs(match self.phase {
            Phase::Focus => profile.focus_minutes * 60,
            Phase::ShortBreak => profile.short_break_minutes * 60,
            Phase::LongBreak => profile.long_break_minutes * 60,
            Phase::Paused | Phase::Finished => 0,
        });
    }

    fn initial_total(&self) -> Duration {
        match &self.mode {
            TimerMode::Pomodoro { profile } => Duration::from_secs(profile.focus_minutes * 60),
            TimerMode::Countdown { seconds } => Duration::from_secs(*seconds),
            TimerMode::Stopwatch => Duration::from_secs(24 * 60 * 60),
            TimerMode::EventCountdown { .. } => Duration::from_secs(self.event_remaining_seconds()),
        }
    }

    fn event_remaining_seconds(&self) -> u64 {
        match &self.mode {
            TimerMode::EventCountdown { at, .. } => {
                (*at - Local::now()).num_seconds().max(0) as u64
            }
            _ => 0,
        }
    }

    fn cycle_target(&self) -> u32 {
        match &self.mode {
            TimerMode::Pomodoro { profile } => profile.long_break_every,
            _ => 1,
        }
    }

    fn label(&self) -> String {
        match &self.mode {
            TimerMode::Pomodoro { .. } => "Pomodoro".to_string(),
            TimerMode::Countdown { .. } => "Countdown".to_string(),
            TimerMode::Stopwatch => "Stopwatch".to_string(),
            TimerMode::EventCountdown { name, .. } => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pomodoro_starts_in_focus() {
        let engine = TimerEngine::pomodoro(Profile::default());
        let snapshot = engine.snapshot();
        assert_eq!(snapshot.phase, Phase::Focus);
        assert_eq!(snapshot.remaining.as_secs(), 25 * 60);
        assert_eq!(snapshot.cycle, 1);
    }

    #[test]
    fn skip_focus_goes_to_short_break() {
        let mut engine = TimerEngine::pomodoro(Profile::default());
        engine.skip();
        let snapshot = engine.snapshot();
        assert_eq!(snapshot.phase, Phase::ShortBreak);
        assert_eq!(snapshot.remaining.as_secs(), 5 * 60);
    }

    #[test]
    fn fourth_focus_goes_to_long_break() {
        let mut engine = TimerEngine::pomodoro(Profile::default());
        for _ in 0..3 {
            engine.skip();
            engine.skip();
        }
        engine.skip();
        assert_eq!(engine.snapshot().phase, Phase::LongBreak);
    }

    #[test]
    fn pause_preserves_phase() {
        let mut engine = TimerEngine::pomodoro(Profile::default());
        engine.start();
        engine.pause();
        assert_eq!(engine.snapshot().phase, Phase::Paused);
        engine.start();
        assert_eq!(engine.snapshot().phase, Phase::Focus);
    }
}
