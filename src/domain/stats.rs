use chrono::{Local, NaiveDate, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::session::SessionRecord;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsSummary {
    pub pomodoros_today: usize,
    pub focus_minutes_today: u64,
    pub completed_sessions: usize,
    pub skipped_sessions: usize,
    pub streak_days: usize,
    pub best_hour: Option<u32>,
    pub most_frequent_task: Option<String>,
    pub by_tag: BTreeMap<String, u64>,
    pub last_7_days: Vec<DayStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStat {
    pub date: NaiveDate,
    pub label: String,
    pub count: usize,
}

impl StatsSummary {
    pub fn from_sessions(records: &[SessionRecord]) -> Self {
        let today = Local::now().date_naive();
        let mut summary = Self::default();
        let mut by_hour = BTreeMap::<u32, usize>::new();
        let mut by_task = BTreeMap::<String, usize>::new();
        let mut by_day = BTreeMap::<NaiveDate, usize>::new();

        for record in records {
            let day = record.ended_at.date_naive();
            if record.completed {
                summary.completed_sessions += 1;
                *by_day.entry(day).or_default() += 1;
            } else {
                summary.skipped_sessions += 1;
            }
            if day == today && record.session_type == "Focus" && record.completed {
                summary.pomodoros_today += 1;
                summary.focus_minutes_today += record.actual_seconds / 60;
            }
            *by_hour.entry(record.ended_at.hour()).or_default() += 1;
            if let Some(task) = &record.task {
                *by_task.entry(task.clone()).or_default() += 1;
            }
            for tag in &record.tags {
                *summary.by_tag.entry(tag.clone()).or_default() += record.actual_seconds / 60;
            }
        }

        summary.best_hour = by_hour
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(hour, _)| hour);
        summary.most_frequent_task = by_task
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(task, _)| task);
        summary.last_7_days = (0..7)
            .rev()
            .map(|offset| {
                let date = today - chrono::Duration::days(offset);
                DayStat {
                    date,
                    label: date.format("%a").to_string(),
                    count: by_day.get(&date).copied().unwrap_or_default(),
                }
            })
            .collect();

        summary.streak_days = streak_days(today, &by_day);
        summary
    }

    pub fn render_text(&self, _today: bool, _week: bool) -> String {
        let mut lines = vec![
            format!("Pomodoros today: {}", self.pomodoros_today),
            format!("Focus minutes today: {}", self.focus_minutes_today),
            format!("Daily streak: {}", self.streak_days),
            format!(
                "Completed / skipped: {} / {}",
                self.completed_sessions, self.skipped_sessions
            ),
        ];
        if let Some(hour) = self.best_hour {
            lines.push(format!("Best hour: {hour:02}:00"));
        }
        if let Some(task) = &self.most_frequent_task {
            lines.push(format!("Most frequent task: {task}"));
        }
        lines.push("Last 7 days:".to_string());
        for day in &self.last_7_days {
            lines.push(format!(
                "{:>3}  {:<10} {}",
                day.label,
                "█".repeat(day.count),
                day.count
            ));
        }
        lines.join("\n")
    }
}

fn streak_days(today: NaiveDate, by_day: &BTreeMap<NaiveDate, usize>) -> usize {
    let mut streak = 0;
    for offset in 0..365 {
        let day = today - chrono::Duration::days(offset);
        if by_day.get(&day).copied().unwrap_or_default() == 0 {
            break;
        }
        streak += 1;
    }
    streak
}
