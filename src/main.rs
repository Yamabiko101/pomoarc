mod app;
mod cli;
mod config;
mod domain;
mod notifications;
mod storage;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, ConfigCommand, SoundCommand, StatsCommand, ThemesCommand};
use config::Config;
use domain::timer::TimerMode;
use storage::JsonlStore;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .without_time()
        .init();

    let cli = Cli::parse();
    let mut config = Config::load_or_create()?;

    if let Some(font) = cli.font {
        config.visuals.font = font;
    }
    if cli.no_animations {
        config.visuals.animations = false;
    }

    match cli.command.unwrap_or(Command::Tui) {
        Command::Tui => app::run_tui(config, cli.profile, cli.task),
        Command::Start { profile, task } => {
            app::run_tui(config, profile.or(cli.profile), task.or(cli.task))
        }
        Command::Countdown { duration } => {
            let seconds = cli::parse_duration(&duration)?;
            app::run_timer_view(config, TimerMode::Countdown { seconds }, None)
        }
        Command::Stopwatch => app::run_timer_view(config, TimerMode::Stopwatch, None),
        Command::Event { name, at } => {
            let at = cli::parse_event_time(&at)?;
            app::run_timer_view(config, TimerMode::EventCountdown { name, at }, None)
        }
        Command::Stats(command) => handle_stats(command),
        Command::Config(command) => handle_config(command, &mut config),
        Command::Themes(command) => handle_themes(command, &mut config),
        Command::Task(command) => app::handle_task(command),
        Command::Notify { test } => {
            if test {
                notifications::notify("pomoarc", "Notification fallback is working.")?;
            }
            Ok(())
        }
        Command::Sound(SoundCommand::Test) => notifications::play_sound(),
    }
}

fn handle_stats(command: StatsCommand) -> Result<()> {
    let store = JsonlStore::new(Config::data_dir()?.join("sessions.jsonl"));
    let stats = store.stats()?;
    if command.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!("{}", stats.render_text(command.today, command.week));
    }
    Ok(())
}

fn handle_config(command: ConfigCommand, config: &mut Config) -> Result<()> {
    match command {
        ConfigCommand::Path => {
            println!("{}", Config::path()?.display());
            Ok(())
        }
        ConfigCommand::Open => config.open_in_editor(),
        ConfigCommand::Reset => {
            Config::write_default()?;
            println!("Config reset at {}", Config::path()?.display());
            Ok(())
        }
        ConfigCommand::Get { key } => {
            println!(
                "{}",
                config
                    .get_key(&key)
                    .unwrap_or_else(|| "<not found>".to_string())
            );
            Ok(())
        }
        ConfigCommand::Set { key, value } => {
            config.set_key(&key, &value)?;
            config.save()?;
            println!("Set {key} = {value}");
            Ok(())
        }
    }
}

fn handle_themes(command: ThemesCommand, config: &mut Config) -> Result<()> {
    match command {
        ThemesCommand::List => {
            for theme in tui::theme::Theme::catalog() {
                println!("{}", theme.name);
            }
            Ok(())
        }
        ThemesCommand::Preview { name } => {
            let theme = tui::theme::Theme::by_name(&name)?;
            println!("{}", theme.preview());
            Ok(())
        }
        ThemesCommand::Set { name } => {
            tui::theme::Theme::by_name(&name)?;
            config.visuals.theme = name.clone();
            config.save()?;
            println!("Theme set to {name}");
            Ok(())
        }
    }
}
