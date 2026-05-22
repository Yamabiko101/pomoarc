use anyhow::{anyhow, Result};
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub warning: Color,
    pub danger: Color,
    pub muted: Color,
    pub border: Color,
    pub progress_full: Color,
    pub progress_empty: Color,
}

type Palette = [&'static str; 11];

impl Theme {
    pub fn catalog() -> Vec<Self> {
        [
            (
                "gruvbox-dark",
                [
                    "#282828", "#ebdbb2", "#83a598", "#d3869b", "#b8bb26", "#fabd2f", "#fb4934",
                    "#928374", "#3c3836", "#b8bb26", "#504945",
                ],
            ),
            (
                "gruvbox-light",
                [
                    "#fbf1c7", "#3c3836", "#076678", "#8f3f71", "#79740e", "#b57614", "#9d0006",
                    "#928374", "#d5c4a1", "#79740e", "#ebdbb2",
                ],
            ),
            (
                "everforest-dark",
                [
                    "#2d353b", "#d3c6aa", "#7fbbb3", "#d699b6", "#a7c080", "#dbbc7f", "#e67e80",
                    "#859289", "#475258", "#a7c080", "#3d484d",
                ],
            ),
            (
                "everforest-light",
                [
                    "#fdf6e3", "#5c6a72", "#3a94c5", "#df69ba", "#8da101", "#dfa000", "#f85552",
                    "#939f91", "#e0dcc7", "#8da101", "#f4f0d9",
                ],
            ),
            (
                "catppuccin-mocha",
                [
                    "#1e1e2e", "#cdd6f4", "#89b4fa", "#f5c2e7", "#a6e3a1", "#f9e2af", "#f38ba8",
                    "#6c7086", "#313244", "#a6e3a1", "#45475a",
                ],
            ),
            (
                "catppuccin-macchiato",
                [
                    "#24273a", "#cad3f5", "#8aadf4", "#f5bde6", "#a6da95", "#eed49f", "#ed8796",
                    "#6e738d", "#363a4f", "#a6da95", "#494d64",
                ],
            ),
            (
                "catppuccin-frappe",
                [
                    "#303446", "#c6d0f5", "#8caaee", "#f4b8e4", "#a6d189", "#e5c890", "#e78284",
                    "#737994", "#414559", "#a6d189", "#51576d",
                ],
            ),
            (
                "catppuccin-latte",
                [
                    "#eff1f5", "#4c4f69", "#1e66f5", "#ea76cb", "#40a02b", "#df8e1d", "#d20f39",
                    "#8c8fa1", "#dce0e8", "#40a02b", "#ccd0da",
                ],
            ),
            (
                "monochrome",
                [
                    "#111111", "#eeeeee", "#ffffff", "#cccccc", "#bbbbbb", "#dddddd", "#ffffff",
                    "#777777", "#555555", "#ffffff", "#333333",
                ],
            ),
            (
                "high-contrast",
                [
                    "#000000", "#ffffff", "#00ffff", "#ff00ff", "#00ff00", "#ffff00", "#ff5555",
                    "#bbbbbb", "#ffffff", "#00ff00", "#333333",
                ],
            ),
        ]
        .into_iter()
        .map(|(name, palette)| theme(name, palette))
        .collect()
    }

    pub fn by_name(name: &str) -> Result<Self> {
        Self::catalog()
            .into_iter()
            .find(|theme| theme.name == name)
            .ok_or_else(|| anyhow!("unknown theme: {name}"))
    }

    pub fn preview(&self) -> String {
        format!(
            "{}\nforeground / primary / accent / warning / danger\n[██████████░░░░░░] 62%",
            self.name
        )
    }
}

fn theme(name: &str, palette: Palette) -> Theme {
    Theme {
        name: name.to_string(),
        background: color(palette[0]),
        foreground: color(palette[1]),
        primary: color(palette[2]),
        secondary: color(palette[3]),
        accent: color(palette[4]),
        warning: color(palette[5]),
        danger: color(palette[6]),
        muted: color(palette[7]),
        border: color(palette[8]),
        progress_full: color(palette[9]),
        progress_empty: color(palette[10]),
    }
}

fn color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    Color::Rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_required_themes() {
        let names: Vec<_> = Theme::catalog()
            .into_iter()
            .map(|theme| theme.name)
            .collect();
        assert!(names.contains(&"gruvbox-dark".to_string()));
        assert!(names.contains(&"everforest-dark".to_string()));
        assert!(names.contains(&"catppuccin-mocha".to_string()));
        assert!(names.contains(&"high-contrast".to_string()));
    }
}
