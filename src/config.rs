use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

use crate::domain::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub timer: TimerConfig,
    pub visuals: VisualConfig,
    pub input: InputConfig,
    pub notifications: NotificationConfig,
    pub storage: StorageConfig,
    pub hooks: HookConfig,
    pub profiles: BTreeMap<String, Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_mode: String,
    pub confirm_on_quit: bool,
    pub start_in_tui: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerConfig {
    pub focus_minutes: u64,
    pub short_break_minutes: u64,
    pub long_break_minutes: u64,
    pub long_break_every: u32,
    pub auto_start_breaks: bool,
    pub auto_start_focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    pub theme: String,
    pub font: String,
    pub animations: bool,
    pub animation_speed: String,
    pub ambient_background: String,
    pub mascot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub mouse: bool,
    pub vim_keys: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub sound: bool,
    pub macos_notification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HookConfig {
    pub enabled: bool,
    pub on_focus_start: String,
    pub on_focus_end: String,
    pub on_break_start: String,
    pub on_break_end: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = BTreeMap::new();
        profiles.insert("default".to_string(), Profile::default());
        profiles.insert("deep-work".to_string(), Profile::deep_work());
        profiles.insert("micro".to_string(), Profile::micro());
        Self {
            app: AppConfig {
                default_mode: "pomodoro".to_string(),
                confirm_on_quit: true,
                start_in_tui: true,
            },
            timer: TimerConfig {
                focus_minutes: 25,
                short_break_minutes: 5,
                long_break_minutes: 15,
                long_break_every: 4,
                auto_start_breaks: false,
                auto_start_focus: false,
            },
            visuals: VisualConfig {
                theme: "catppuccin-mocha".to_string(),
                font: "digital".to_string(),
                animations: true,
                animation_speed: "normal".to_string(),
                ambient_background: "none".to_string(),
                mascot: "tomato".to_string(),
            },
            input: InputConfig {
                mouse: true,
                vim_keys: true,
            },
            notifications: NotificationConfig {
                enabled: true,
                sound: true,
                macos_notification: true,
            },
            storage: StorageConfig {
                backend: "jsonl".to_string(),
            },
            hooks: HookConfig::default(),
            profiles,
        }
    }
}

impl Config {
    pub fn load_or_create() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            Self::write_default()?;
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        toml::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn path() -> Result<PathBuf> {
        if let Some(home) = Self::env_home() {
            return Ok(home.join("config.toml"));
        }
        Ok(Self::project_dirs()?.config_dir().join("config.toml"))
    }

    pub fn data_dir() -> Result<PathBuf> {
        let dir = Self::env_home()
            .map(|home| home.join("data"))
            .unwrap_or(Self::project_dirs()?.data_dir().to_path_buf());
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn write_default() -> Result<()> {
        Self::default().save()
    }

    pub fn profile(&self, name: Option<&str>) -> Profile {
        let key = name.unwrap_or("default");
        self.profiles
            .get(key)
            .cloned()
            .or_else(|| self.profiles.get("default").cloned())
            .unwrap_or_default()
    }

    pub fn get_key(&self, key: &str) -> Option<String> {
        match key {
            "theme.name" | "visual.theme" | "visuals.theme" => Some(self.visuals.theme.clone()),
            "visual.font" | "visuals.font" => Some(self.visuals.font.clone()),
            "timer.focus_minutes" => Some(self.timer.focus_minutes.to_string()),
            "timer.short_break_minutes" => Some(self.timer.short_break_minutes.to_string()),
            "timer.long_break_minutes" => Some(self.timer.long_break_minutes.to_string()),
            "input.mouse" => Some(self.input.mouse.to_string()),
            "input.vim_keys" => Some(self.input.vim_keys.to_string()),
            "storage.backend" => Some(self.storage.backend.clone()),
            _ => None,
        }
    }

    pub fn set_key(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "theme.name" | "visual.theme" | "visuals.theme" => {
                self.visuals.theme = value.to_string()
            }
            "visual.font" | "visuals.font" => self.visuals.font = value.to_string(),
            "timer.focus_minutes" => self.timer.focus_minutes = value.parse()?,
            "timer.short_break_minutes" => self.timer.short_break_minutes = value.parse()?,
            "timer.long_break_minutes" => self.timer.long_break_minutes = value.parse()?,
            "input.mouse" => self.input.mouse = value.parse()?,
            "input.vim_keys" => self.input.vim_keys = value.parse()?,
            "storage.backend" => self.storage.backend = value.to_string(),
            _ => return Err(anyhow!("unsupported config key: {key}")),
        }
        Ok(())
    }

    pub fn open_in_editor(&self) -> Result<()> {
        let path = Self::path()?;
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "open".to_string());
        Command::new(editor).arg(path).status()?;
        Ok(())
    }

    fn project_dirs() -> Result<ProjectDirs> {
        ProjectDirs::from("dev", "Pomoarc", "pomoarc")
            .ok_or_else(|| anyhow!("could not determine project directories"))
    }

    fn env_home() -> Option<PathBuf> {
        std::env::var_os("POMOARC_HOME").map(PathBuf::from)
    }
}
