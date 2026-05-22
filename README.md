<div align="center">

# pomoarc

**A quiet, polished Pomodoro TUI with Apple-like calm, Omarchy terminal energy, and Nothing-style restraint.**

`Rust` · `Ratatui` · `Crossterm` · `macOS-first` · `keyboard + mouse`

```text
┌────────────────────────────────────────────────────────────────────┐
│ pomoarc                                                    25:00   │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│        ████  █████        ███   ███                               │
│            █ █       █   █   █ █   █                              │
│         ███  ████        █   █ █   █                              │
│        █         █   █   █   █ █   █                              │
│        █████ ████         ███   ███                               │
│                                                                    │
│        [████████████░░░░░░░░░░░░░░░░] 48%                          │
│                                                                    │
│        focus garden      .                                         │
│        active task       Write README                              │
│        theme             catppuccin-mocha                          │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

[Install](#install) · [Use](#quick-use) · [Themes](#themes) · [Development](#development) · [Roadmap](#roadmap)

</div>

## Mood

pomoarc is a terminal focus companion, not a productivity dashboard shouting at you.

It aims for:

- **Calm by default**: readable, low-friction, no visual noise.
- **Terminal-native charm**: ASCII timer, keyboard flow, mouse when available.
- **Small rituals**: cycles, garden progress, a simple mascot, session stats.
- **Serious tooling**: real CLI commands, local config, tests, installable binary.

## What Works Now

| Area | Status |
| --- | --- |
| Pomodoro TUI | Implemented |
| Start, pause/resume, reset, skip, quit | Implemented |
| CLI commands | Implemented |
| Themes | 10 built in |
| ASCII timer | Implemented with compact fallback |
| Mouse support | Basic click zones |
| Config | TOML, platform app path |
| Stats | JSONL-backed summary and JSON export |
| Tasks | CLI add/list |
| macOS notifications | `terminal-notifier` or `osascript` fallback |
| Sound | `afplay` fallback |

Experimental:

- Stats, tasks and settings tabs inside the TUI are informational first versions.
- Theme and font switching inside the TUI is session-local.
- Focus Garden and mascot visuals are intentionally simple in this release.
- SQLite, ritual mode, hooks and richer ambient scenes are roadmap items.

## Install

Install Rust if needed:

```bash
brew install rust
```

Build and install from the repo:

```bash
cargo install --path .
pomolife
```

The installed command is currently:

```bash
pomolife
```

If your shell cannot find it:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
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

## Keyboard

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

## Mouse

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

pomoarc ships with calm, readable terminal palettes:

| Family | Themes |
| --- | --- |
| Gruvbox | `gruvbox-dark`, `gruvbox-light` |
| Everforest | `everforest-dark`, `everforest-light` |
| Catppuccin | `catppuccin-mocha`, `catppuccin-macchiato`, `catppuccin-frappe`, `catppuccin-latte` |
| Utility | `monochrome`, `high-contrast` |

Preview and set:

```bash
pomolife themes preview catppuccin-mocha
pomolife themes set everforest-dark
```

```text
catppuccin-mocha  #1e1e2e  #cdd6f4  #89b4fa  #a6e3a1
everforest-dark   #2d353b  #d3c6aa  #7fbbb3  #a7c080
gruvbox-dark      #282828  #ebdbb2  #83a598  #b8bb26
```

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
theme = "catppuccin-mocha"
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
- SQLite backend and migrations.
- Full task picker and profile picker inside the TUI.
- Ritual mode with energy check-in, intention and micro-journaling.
- Ambient backgrounds: stars, rain, garden, matrix and waves.
- External theme files and theme validation.
- Snapshot tests for small terminals.
- Homebrew tap packaging.

## Contributing

Keep the timer engine independent from Ratatui, keep the TUI responsive, and run the development checks before opening a PR.
