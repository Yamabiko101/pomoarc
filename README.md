<div align="center">

# pomoarc

**A dark terminal focus system for people who tune their workstation like a living environment.**

Omarchy-coded · terminal-native · keyboard-first · local-only · built in Rust

![Rust](https://img.shields.io/badge/Rust-111111?style=for-the-badge&logo=rust&logoColor=white)
![Ratatui](https://img.shields.io/badge/Ratatui-7fbbb3?style=for-the-badge&labelColor=111111)
![Omarchy](https://img.shields.io/badge/Omarchy-2d353b?style=for-the-badge&labelColor=111111&logoColor=a7c080)
![Terminal](https://img.shields.io/badge/Terminal-1e1e2e?style=for-the-badge&logo=gnometerminal&logoColor=a6e3a1)
![Private Build](https://img.shields.io/badge/private_build-pomoarc-a6e3a1?style=for-the-badge&labelColor=111111)

```text
╭────────────────────────────── pomoarc ──────────────────────────────╮
│ mode: focus       profile: deep-work       theme: everforest-dark    │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│      ██████╗  ██████╗ ███╗   ███╗ ██████╗  █████╗ ██████╗  ██████╗ │
│      ██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██╔══██╗██╔══██╗██╔════╝ │
│      ██████╔╝██║   ██║██╔████╔██║██║   ██║███████║██████╔╝██║      │
│      ██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██╔══██║██╔══██╗██║      │
│      ██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝██║  ██║██║  ██║╚██████╗ │
│      ╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ │
│                                                                      │
│                  25:00        scanline: soft                         │
│              [██████████████▓▓▓▓▒▒▒▒░░░░] 62%                       │
│                                                                      │
│   task       write the thing         state      locked in             │
│   ritual     breathe / choose / ship  energy     steady               │
│   garden     .  |  \|/  \|/_         signal     green                │
│   controls   s start · space pause · n skip · t theme · q quit       │
╰──────────────────────────────────────────────────────────────────────╯
```

`pomolife` today. `pomoarc` as the product identity.

[Install](#install) · [Themes](#theme-system) · [Controls](#controls) · [Visual Direction](#visual-direction) · [Roadmap](#roadmap)

</div>

## Terminal Moodboard

pomoarc should feel less like a calendar app and more like a tuned terminal rice:

```text
black glass        #111111  ████████████████████  base
everforest green   #a7c080  ████████████░░░░░░░░  growth
terminal teal      #7fbbb3  ██████████████░░░░░░  focus
signal pink        #f5c2e7  ████████░░░░░░░░░░░░  accent
warning amber      #f9e2af  ██████████░░░░░░░░░░  break
danger red         #f38ba8  ██████░░░░░░░░░░░░░░  skip
```

Design language:

- **Omarchy first**: sharp borders, dark surfaces, terminal color, no soft landing-page gloss.
- **Riced but useful**: visual detail only where it helps focus.
- **Neon signal, not decoration**: progress, active state, warning, theme and completion.
- **ASCII as atmosphere**: timer glyphs, garden frames, mascot states and subtle motion.
- **Hackable private tool energy**: compact, opinionated, local, easy to fork.

## Motion Language

The README cannot animate the terminal, so this is the intended terminal pulse:

```text
tick 001  [██████████░░░░░░░░░░]  focus  signal: green
tick 002  [██████████▓░░░░░░░░░]  focus  signal: green
tick 003  [██████████▒▒░░░░░░░░]  focus  signal: green
tick 004  [██████████░░░░░░░░░░]  focus  signal: green
```

Inside the app, animations are intentionally low-power and can be disabled:

```bash
pomolife --no-animations
```

## What Works Now

| System | Current state |
| --- | --- |
| TUI Pomodoro | Start, pause/resume, reset, skip, quit |
| Terminal UI | Ratatui layout, ASCII timer, progress bar |
| Keyboard | Full first-pass control map |
| Mouse | Basic clickable tabs and footer controls |
| Themes | 10 built-in palettes |
| Fonts | 8 font names with compact fallback |
| Profiles | `default`, `deep-work`, `micro` |
| Config | TOML, platform app path |
| Storage | JSONL sessions and tasks |
| Stats | Text and JSON summary |
| macOS | Notifications and sound fallbacks |

Experimental:

- TUI tabs for stats/tasks/settings are informational first versions.
- Theme and font switching inside the TUI is session-local.
- Focus Garden and mascot visuals are simple seeds, ready for richer animation.
- SQLite, ritual mode, hooks and ambient backgrounds are next-phase work.

## Install

From source:

```bash
brew install rust
git clone https://github.com/Yamabiko101/pomoarc.git
cd pomoarc
cargo install --path .
pomolife
```

## Quick Use

```bash
pomolife
pomolife start
pomolife start --profile deep-work --task "Write README"
pomolife countdown 10m
pomolife stopwatch
pomolife event "Project delivery" --at "2026-06-01 09:00"
pomolife stats
pomolife stats --json
```

## Command Map

```bash
pomolife --help
pomolife tui
pomolife start --profile micro
pomolife task add "Read paper" --tag reading
pomolife task list
pomolife themes list
pomolife themes preview catppuccin-mocha
pomolife themes set everforest-dark
pomolife config path
pomolife config get visual.theme
pomolife config set timer.focus_minutes 50
pomolife notify --test
pomolife sound test
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

## Theme System

Built-in palettes:

| Family | Themes | Vibe |
| --- | --- | --- |
| Gruvbox | `gruvbox-dark`, `gruvbox-light` | warm terminal classic |
| Everforest | `everforest-dark`, `everforest-light` | Omarchy garden mode |
| Catppuccin | `mocha`, `macchiato`, `frappe`, `latte` | soft neon workstation |
| Utility | `monochrome`, `high-contrast` | no-distraction mode |

Preview and set:

```bash
pomolife themes preview catppuccin-mocha
pomolife themes set everforest-dark
```

Palette strip:

```text
everforest-dark   bg #2d353b  fg #d3c6aa  primary #7fbbb3  grow #a7c080
catppuccin-mocha  bg #1e1e2e  fg #cdd6f4  primary #89b4fa  grow #a6e3a1
gruvbox-dark      bg #282828  fg #ebdbb2  primary #83a598  grow #b8bb26
high-contrast     bg #000000  fg #ffffff  primary #00ffff  grow #00ff00
```

## Visual Direction

Planned presentation layers for the TUI:

```text
╭─ focus surface ─────────────────────────────────────╮
│  dense, dark, clear hierarchy                       │
│  active region uses green/teal signal color          │
│  breaks use amber, skips/errors use red              │
│  optional low-power scanline/progress pulse          │
╰─────────────────────────────────────────────────────╯
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

Use:

```bash
pomolife --font tiny
```

Small terminals automatically fall back to a compact timer.

## Profiles

Built-in focus rhythms:

| Profile | Focus | Short break | Long break | Long break every |
| --- | ---: | ---: | ---: | ---: |
| `default` | 25m | 5m | 15m | 4 |
| `deep-work` | 50m | 10m | 25m | 3 |
| `micro` | 10m | 2m | 8m | 5 |

```bash
pomolife start --profile deep-work
```

## Config

Print the config path:

```bash
pomolife config path
```

On macOS this uses the system application config directory through the `directories` crate. Commonly this resolves under:

```text
~/Library/Application Support/dev.Pomolife.pomolife/config.toml
```

For isolated development or tests:

```bash
POMOLIFE_HOME=.pomolife-dev pomolife config path
```

Example values:

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

pomoarc stores sessions and tasks as JSONL in the platform data directory.

```bash
pomolife stats
pomolife stats --today
pomolife stats --week
pomolife stats --json
```

Stats include:

- Pomodoros completed today.
- Focus minutes today.
- Daily streak.
- Best hour.
- Most frequent task.
- Tag totals.
- Last 7 days as an ASCII chart.

## macOS Notifications

Completion notifications try, in order:

1. `terminal-notifier`, when installed.
2. `osascript` notification fallback.
3. TUI-only completion if notification commands fail.

Sound uses `afplay` with system sounds.

```bash
pomolife notify --test
pomolife sound test
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

Run:

```bash
reset
```

pomoarc restores raw mode, alternate screen, mouse capture and cursor on normal exit. A terminal crash can still leave the shell in a bad state.

### Colors look wrong

Use a modern terminal with truecolor support:

```bash
export COLORTERM=truecolor
```

### Mouse does not work

Mouse support depends on the terminal. Disable it with:

```toml
[input]
mouse = false
```

### Notifications are silent

Run:

```bash
pomolife notify --test
pomolife sound test
```

### The ASCII font does not fit

Use:

```bash
pomolife --font tiny
```

## Roadmap

Next layers of polish:

- Rename the binary from `pomolife` to `pomoarc` once the product name fully settles.
- Richer Omarchy-style ambient backgrounds: stars, rain, garden, scanline, matrix.
- Animated focus garden states and sharper mascot frames.
- SQLite backend and migrations.
- Full task picker and profile picker inside the TUI.
- Ritual mode with energy check-in, intention and micro-journaling.
- External theme files and theme validation.
- Snapshot tests for small terminals.
- Homebrew tap packaging.

## Contributing

Keep the timer engine independent from Ratatui, keep the TUI responsive, and run the development checks before opening a PR.
