<div align="center">

# pomoarc

**A dark terminal Pomodoro system for focused work.**

Terminal-native · keyboard-first · local-only · built in Rust

![Rust](https://img.shields.io/badge/Rust-111111?style=for-the-badge&logo=rust&logoColor=white)
![Ratatui](https://img.shields.io/badge/Ratatui-7fbbb3?style=for-the-badge&labelColor=111111)
![Terminal](https://img.shields.io/badge/Terminal-1e1e2e?style=for-the-badge&logo=gnometerminal&logoColor=a6e3a1)
![Local First](https://img.shields.io/badge/Local_First-a6e3a1?style=for-the-badge&labelColor=111111)

```text
╭────────────────────────────── pomoarc ──────────────────────────────╮
│ mode: focus       profile: deep-work       theme: everforest-dark    │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│      ██████╗  ██████╗ ███╗   ███╗ ██████╗  █████╗ ██████╗  ██████╗   │
│      ██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██╔══██╗██╔══██╗██╔════╝   │
│      ██████╔╝██║   ██║██╔████╔██║██║   ██║███████║██████╔╝██║        │
│      ██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██╔══██║██╔══██╗██║        │
│      ██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝██║  ██║██║  ██║╚██████╗   │
│      ╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝   │
│                                                                      │
│                  25:00        signal: focus                          │
│              [██████████████▓▓▓▓▒▒▒▒░░░░] 62%                        │
│                                                                      │
│   task       write the thing         state      locked in            │
│   garden     .  |  \|/  \|/_         rhythm     deep-work            │
│   controls   s start · space pause · n skip · t theme · q quit       │
╰──────────────────────────────────────────────────────────────────────╯
```

[Install](#install) · [Usage](#usage) · [Themes](#themes) · [Controls](#controls) · [Roadmap](#roadmap)

</div>

## Overview

pomoarc is a terminal-first focus timer with a clean TUI, local configuration, keyboard control, mouse support where available, and persistent stats.

The project is designed around a dark workstation aesthetic: high contrast, sharp terminal borders, readable state, and subtle motion.

## Current Status

| Area | Status |
| --- | --- |
| Pomodoro TUI | Start, pause/resume, reset, skip, quit |
| CLI | `start`, `tui`, `countdown`, `stopwatch`, `event`, `stats`, `config`, `themes`, `task`, `notify`, `sound` |
| Themes | 10 built-in palettes |
| Fonts | 8 font names with compact fallback |
| Input | Keyboard controls and basic mouse click zones |
| Config | TOML config using platform app paths |
| Storage | Local JSONL sessions and tasks |
| Stats | Text summary and JSON export |
| macOS | Notification and sound fallbacks |

Experimental:

- Stats, tasks, and settings tabs inside the TUI are informational first versions.
- Theme and font switching inside the TUI is session-local.
- Focus Garden and mascot visuals are early visual seeds.

## Install

```bash
brew install rust
git clone https://github.com/Yamabiko101/pomoarc.git
cd pomoarc
cargo install --path .
pomoarc
```

If the shell cannot find `pomoarc`, add Cargo's bin directory to your path:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

```bash
pomoarc
pomoarc start
pomoarc start --profile deep-work --task "Write README"
pomoarc countdown 10m
pomoarc stopwatch
pomoarc event "Project delivery" --at "2026-06-01 09:00"
pomoarc stats
pomoarc stats --json
```

## Command Map

```bash
pomoarc --help
pomoarc tui
pomoarc start --profile micro
pomoarc task add "Read paper" --tag reading
pomoarc task list
pomoarc themes list
pomoarc themes preview catppuccin-mocha
pomoarc themes set everforest-dark
pomoarc config path
pomoarc config get visual.theme
pomoarc config set timer.focus_minutes 50
pomoarc notify --test
pomoarc sound test
```

## Controls

| Key | Action |
| --- | --- |
| `Space` | Pause / resume |
| `s` | Start |
| `r` | Reset session |
| `n` | Skip phase |
| `q` | Quit |
| `?` | Help |
| `t` | Next theme |
| `m` | Next tab |
| `a` | Next ASCII font |
| `+` | Add 1 minute |
| `-` | Remove 1 minute |
| `Tab` | Next panel |
| `Esc` | Quit / close modal |

Mouse support is enabled by default when the terminal supports it.

Clickable areas:

- Footer controls: Start, Pause, Reset, Skip, Theme, Help.
- Top tabs: Timer, Stats, Tasks, Settings.

Disable mouse input:

```toml
[input]
mouse = false
```

## Themes

Built-in palettes:

| Family | Themes | Use |
| --- | --- | --- |
| Gruvbox | `gruvbox-dark`, `gruvbox-light` | warm terminal classic |
| Everforest | `everforest-dark`, `everforest-light` | green, quiet, focused |
| Catppuccin | `catppuccin-mocha`, `catppuccin-macchiato`, `catppuccin-frappe`, `catppuccin-latte` | soft contrast |
| Utility | `monochrome`, `high-contrast` | reduced distraction |

```bash
pomoarc themes preview catppuccin-mocha
pomoarc themes set everforest-dark
```

```text
everforest-dark   bg #2d353b  fg #d3c6aa  primary #7fbbb3  grow #a7c080
catppuccin-mocha  bg #1e1e2e  fg #cdd6f4  primary #89b4fa  grow #a6e3a1
gruvbox-dark      bg #282828  fg #ebdbb2  primary #83a598  grow #b8bb26
high-contrast     bg #000000  fg #ffffff  primary #00ffff  grow #00ff00
```

## Visual System

```text
black glass        #111111  ████████████████████  base
everforest green   #a7c080  ████████████░░░░░░░░  growth
terminal teal      #7fbbb3  ██████████████░░░░░░  focus
signal pink        #f5c2e7  ████████░░░░░░░░░░░░  accent
warning amber      #f9e2af  ██████████░░░░░░░░░░  break
danger red         #f38ba8  ██████░░░░░░░░░░░░░░  skip
```

Motion target:

```text
tick 001  [██████████░░░░░░░░░░]  focus
tick 002  [██████████▓░░░░░░░░░]  focus
tick 003  [██████████▒▒░░░░░░░░]  focus
tick 004  [██████████░░░░░░░░░░]  focus
```

Disable animations:

```bash
pomoarc --no-animations
```

Future image slots:

| Slot | Purpose |
| --- | --- |
| `hero-terminal.png` | Main TUI screenshot |
| `themes-strip.png` | Theme comparison |
| `garden-states.png` | Focus Garden progression |
| `compact-mode.png` | Small terminal fallback |

## ASCII Fonts

Available font names:

- `digital`
- `block`
- `tiny`
- `minimal`
- `rounded`
- `shadow`
- `big`
- `slant`

```bash
pomoarc --font tiny
```

Small terminals automatically fall back to a compact timer.

## Profiles

| Profile | Focus | Short break | Long break | Long break every |
| --- | ---: | ---: | ---: | ---: |
| `default` | 25m | 5m | 15m | 4 |
| `deep-work` | 50m | 10m | 25m | 3 |
| `micro` | 10m | 2m | 8m | 5 |

```bash
pomoarc start --profile deep-work
```

## Config

```bash
pomoarc config path
```

Common macOS path:

```text
~/Library/Application Support/dev.Pomoarc.pomoarc/config.toml
```

For isolated development or tests:

```bash
POMOARC_HOME=.pomoarc-dev pomoarc config path
```

Example:

```toml
[visuals]
theme = "everforest-dark"
font = "digital"
animations = true
ambient_background = "none"

[input]
mouse = true
vim_keys = true
```

## Stats

```bash
pomoarc stats
pomoarc stats --today
pomoarc stats --week
pomoarc stats --json
```

Stats include completed Pomodoros, focus minutes, streak, best hour, frequent task, tag totals, and a 7-day ASCII chart.

## macOS Notifications

Completion notifications try, in order:

1. `terminal-notifier`, when installed.
2. `osascript` notification fallback.
3. TUI-only completion if notification commands fail.

Sound uses `afplay` with system sounds.

```bash
pomoarc notify --test
pomoarc sound test
```

Optional:

```bash
brew install terminal-notifier
```

## Development

```bash
cargo fetch
cargo build
cargo test
cargo fmt --check
cargo clippy -- -D warnings
cargo run -- --help
cargo run -- stats --json
cargo install --path .
```

Architecture:

```text
src/
  cli.rs          command surface
  config.rs       TOML config and platform paths
  domain/         timer, profiles, sessions, tasks, stats
  storage/        JSONL persistence
  tui/            Ratatui renderer, events, mouse, themes, ASCII
  notifications/  macOS sound and notification fallbacks
```

## Troubleshooting

### The terminal looks broken after exit

```bash
reset
```

pomoarc restores raw mode, alternate screen, mouse capture, and cursor on normal exit.

### Colors look wrong

```bash
export COLORTERM=truecolor
```

### Mouse does not work

```toml
[input]
mouse = false
```

### Notifications are silent

```bash
pomoarc notify --test
pomoarc sound test
```

### The ASCII font does not fit

```bash
pomoarc --font tiny
```

## Roadmap

- Richer ambient backgrounds: stars, rain, garden, scanline, matrix.
- Animated focus garden states and sharper mascot frames.
- SQLite backend and migrations.
- Full task picker and profile picker inside the TUI.
- Ritual mode with energy check-in, intention and micro-journaling.
- External theme files and theme validation.
- Snapshot tests for small terminals.
- Homebrew tap packaging.

## Contributing

Keep the timer engine independent from Ratatui, keep the TUI responsive, and run the development checks before opening a PR.
