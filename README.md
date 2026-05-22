# Pomolife

Pomolife is a lively Pomodoro terminal app for macOS, built with Rust, Ratatui and Crossterm.

```text
████  ████  █   █  ████  █     █ ████ ████
█  █  █  █  ██ ██  █  █  █     █ █    █
████  █  █  █ █ █  █  █  █     █ ███  ███
█     █  █  █   █  █  █  █     █ █    █
█     ████  █   █  ████  ████  █ █    ████
```

## Status

Implemented in this release:

- Pomodoro TUI with start, pause/resume, reset, skip and quit.
- CLI commands for `start`, `tui`, `countdown`, `stopwatch`, `event`, `stats`, `config`, `themes`, `task`, `notify` and `sound`.
- Timer domain tests.
- Ten built-in themes: Gruvbox, Everforest, Catppuccin, Monochrome and High Contrast.
- ASCII timer fonts with compact fallback.
- Mouse click zones for tabs and main controls when the terminal supports mouse events.
- Local config at the platform app config path.
- JSONL session/task persistence and stats export.
- macOS notification fallback through `terminal-notifier` or `osascript`, and sound through `afplay`.

Experimental in this release:

- TUI tabs for stats/tasks/settings are informational.
- Theme/font switching in the TUI is in-memory for the active session.
- Focus Garden, mascot and ambient visuals are simple first versions.
- SQLite backend and hooks are documented as roadmap; storage currently uses JSONL.

## Install

Install Rust first if needed:

```bash
brew install rust
```

Then build and install:

```bash
cargo install --path .
pomolife
```

If `pomolife` is not found after install, ensure Cargo's bin directory is in your shell path:

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

## Commands

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

Mouse support is enabled by default and depends on the terminal emulator. Click the footer controls for Start, Pause, Reset, Skip, Theme and Help, or click the top tabs.

Disable it in config:

```toml
[input]
mouse = false
```

## Themes

Available themes:

- `gruvbox-dark`
- `gruvbox-light`
- `everforest-dark`
- `everforest-light`
- `catppuccin-mocha`
- `catppuccin-macchiato`
- `catppuccin-frappe`
- `catppuccin-latte`
- `monochrome`
- `high-contrast`

## Fonts

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

## Config

Print the config path:

```bash
pomolife config path
```

On macOS this uses the system application config directory through the `directories` crate. Commonly this resolves under:

```text
~/Library/Application Support/dev.Pomolife.pomolife/config.toml
```

For isolated development or tests, override local app state:

```bash
POMOLIFE_HOME=.pomolife-dev pomolife config path
```

Profiles included by default:

- `default`: 25/5/15, long break every 4 focus sessions.
- `deep-work`: 50/10/25, long break every 3 focus sessions.
- `micro`: 10/2/8, long break every 5 focus sessions.

## Stats

Pomolife stores sessions and tasks as JSONL in the platform data directory.

```bash
pomolife stats
pomolife stats --today
pomolife stats --week
pomolife stats --json
```

Stats include today's completed pomodoros, focus minutes, streak, best hour, frequent task, tag totals and a 7-day ASCII chart.

## macOS Notifications

Pomolife tries:

1. `terminal-notifier`, when installed.
2. `osascript` notification fallback.
3. TUI-only completion if notification commands fail.

Sound uses `afplay` with system sounds.

```bash
pomolife notify --test
pomolife sound test
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

## Troubleshooting

### The terminal looks broken after exit

Run:

```bash
reset
```

Pomolife restores raw mode, alternate screen, mouse capture and cursor on normal exit. A terminal crash can still leave the shell in a bad state.

### Colors look wrong

Use a modern terminal with truecolor support and try:

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

Install optional notification support:

```bash
brew install terminal-notifier
```

### The ASCII font does not fit

Use:

```bash
pomolife --font tiny
```

## Roadmap

- SQLite backend and migrations.
- Full task picker and profile picker inside the TUI.
- Ritual mode with energy check-in, intention and micro-journaling.
- Ambient backgrounds: stars, rain, garden, matrix and waves.
- External theme files and theme validation.
- Snapshot tests for small terminals.
- Homebrew tap packaging.

## Contributing

Keep the timer engine independent from Ratatui, keep the TUI responsive, and run the development checks before opening a PR.
