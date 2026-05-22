use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "pomoarc",
    version,
    about = "A dark terminal Pomodoro TUI for focus sessions"
)]
pub struct Cli {
    #[arg(long, global = true)]
    pub profile: Option<String>,
    #[arg(long, global = true)]
    pub task: Option<String>,
    #[arg(long, global = true)]
    pub font: Option<String>,
    #[arg(long, global = true)]
    pub no_animations: bool,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Start {
        #[arg(long)]
        profile: Option<String>,
        #[arg(long)]
        task: Option<String>,
    },
    Tui,
    Countdown {
        duration: String,
    },
    Stopwatch,
    Event {
        name: String,
        #[arg(long)]
        at: String,
    },
    Stats(StatsCommand),
    #[command(subcommand)]
    Config(ConfigCommand),
    #[command(subcommand)]
    Themes(ThemesCommand),
    #[command(subcommand)]
    Task(TaskCommand),
    #[command(subcommand)]
    Note(NoteCommand),
    Notify {
        #[arg(long)]
        test: bool,
    },
    #[command(subcommand)]
    Sound(SoundCommand),
}

#[derive(Debug, Args)]
pub struct StatsCommand {
    #[arg(long)]
    pub today: bool,
    #[arg(long)]
    pub week: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    Path,
    Open,
    Reset,
    Get { key: String },
    Set { key: String, value: String },
}

#[derive(Debug, Subcommand)]
pub enum ThemesCommand {
    List,
    Preview { name: String },
    Set { name: String },
}

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
    Add {
        title: String,
        #[arg(long)]
        tag: Option<String>,
    },
    List,
    Complete {
        title: String,
    },
    Delete {
        title: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum NoteCommand {
    Add { body: String },
    List,
    Complete { id: u64 },
    Edit { id: u64, body: String },
    Delete { id: u64 },
}

#[derive(Debug, Subcommand)]
pub enum SoundCommand {
    Test,
}

pub fn parse_duration(value: &str) -> Result<u64> {
    let (number, unit) = value.trim().split_at(value.len().saturating_sub(1));
    let amount: u64 = number
        .parse()
        .context("duration must start with a number")?;
    match unit {
        "s" => Ok(amount),
        "m" => Ok(amount * 60),
        "h" => Ok(amount * 60 * 60),
        _ => Err(anyhow!("duration must end in s, m, or h")),
    }
}

pub fn parse_event_time(value: &str) -> Result<DateTime<Local>> {
    let naive = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M")
        .context("expected date format YYYY-MM-DD HH:MM")?;
    Local
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow!("event time is ambiguous in the local timezone"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_duration_units() {
        assert_eq!(parse_duration("90s").unwrap(), 90);
        assert_eq!(parse_duration("10m").unwrap(), 600);
        assert_eq!(parse_duration("2h").unwrap(), 7200);
    }
}
