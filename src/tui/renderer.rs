use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::domain::timer::TimerSnapshot;

use super::{animation, ascii_font, mouse::MouseAction, Tab, UiState};

pub fn render(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot) {
    let area = frame.area();
    let theme = state.theme();
    frame.render_widget(Clear, area);

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(7),
        ])
        .split(area);

    state.mouse_zones_mut().clear();
    render_header(frame, state, root[0]);
    render_body(frame, state, snapshot, root[1]);
    render_footer(frame, state, snapshot, root[2]);

    if state.show_help() {
        render_help(frame, centered_rect(70, 62, area));
    }

    let _ = theme;
}

fn render_header(frame: &mut Frame, state: &mut UiState, area: Rect) {
    let theme = state.theme();
    let titles = ["Timer", "Stats", "Tasks", "Settings"]
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let selected = match state.tab() {
        Tab::Timer => 0,
        Tab::Stats => 1,
        Tab::Tasks => 2,
        Tab::Settings => 3,
    };
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(theme.border)),
        )
        .select(selected)
        .style(Style::default().fg(theme.muted).bg(theme.background))
        .highlight_style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs, area);

    let tab_width = area.width / 4;
    state.mouse_zones_mut().add(
        Rect::new(area.x, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Timer),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Stats),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width * 2, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Tasks),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width * 3, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Settings),
    );
}

fn render_body(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot, area: Rect) {
    match state.tab() {
        Tab::Timer => render_timer(frame, state, snapshot, area),
        Tab::Stats => render_panel(frame, state, area, "Stats", "Run `pomoarc stats --json` for persisted stats.\nToday, streak, best hour and tag totals are stored locally."),
        Tab::Tasks => render_panel(frame, state, area, "Tasks", "Use `pomoarc task add \"Write README\" --tag writing`.\nThe TUI task selector is experimental in this release."),
        Tab::Settings => render_panel(frame, state, area, "Settings", "Config lives at `pomoarc config path`.\nHotkeys: t theme, a font, m tab, ? help."),
    }
}

fn render_timer(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot, area: Rect) {
    let theme = state.theme();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(area);
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(7),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .split(chunks[0]);

    let time = format_duration(snapshot.remaining);
    let ascii = ascii_font::render_time(&time, state.font(), left[0].width);
    let timer = Paragraph::new(ascii.join("\n"))
        .block(
            Block::default()
                .title(snapshot.label.as_str())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        )
        .style(Style::default().fg(theme.primary).bg(theme.background))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    frame.render_widget(timer, left[0]);

    let progress = if snapshot.total.as_secs() == 0 {
        0.0
    } else {
        snapshot.elapsed.as_secs_f64() / snapshot.total.as_secs_f64()
    };
    let bar = animation::progress_bar(
        progress,
        left[1].width.saturating_sub(12) as usize,
        state.frame(),
        true,
    );
    let progress_text = format!("[{bar}] {:>3}%", (progress * 100.0).round() as u8);
    let progress_widget = Paragraph::new(progress_text)
        .block(
            Block::default()
                .title("Progress")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.progress_empty)),
        )
        .style(Style::default().fg(theme.progress_full));
    frame.render_widget(progress_widget, left[1]);

    let status = format!(
        "{} | cycle {} / {} | {} | theme {} | font {}",
        if snapshot.running { "Running" } else { "Ready" },
        snapshot.cycle,
        snapshot.cycle_target,
        snapshot.phase.label(),
        state.theme_name(),
        state.font()
    );
    frame.render_widget(
        Paragraph::new(status)
            .block(
                Block::default()
                    .title("Session")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            )
            .style(Style::default().fg(theme.foreground)),
        left[2],
    );

    render_sidebar(frame, state, snapshot, chunks[1]);
}

fn render_sidebar(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot, area: Rect) {
    let theme = state.theme();
    let garden = match snapshot.cycle.min(5) {
        0 | 1 => ".",
        2 => "|\n|",
        3 => "\\|/\n |",
        4 => "\\|/\n\\|/",
        _ => "\\|/\n\\|/_",
    };
    let mascot = match snapshot.phase.label() {
        "Focus" => "  _____\n / ___ \\\n| |   | |  focus\n \\_____/ ",
        "Paused" => "  _____\n / - - \\\n|  zZ  |  paused\n \\_____/ ",
        _ => "  _____\n / o o \\\n|  v   |  break\n \\_____/ ",
    };
    let text = vec![
        Line::from(vec![
            Span::styled("Task: ", Style::default().fg(theme.warning)),
            Span::raw(state.task()),
        ]),
        Line::raw(""),
        Line::styled(
            "Focus garden",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(garden),
        Line::raw(""),
        Line::styled(
            "Mascot",
            Style::default()
                .fg(theme.secondary)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(mascot),
    ];
    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title("pomoarc")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            )
            .style(Style::default().fg(theme.foreground))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_footer(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot, area: Rect) {
    let theme = state.theme();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Min(10),
        ])
        .split(area);
    let buttons = [
        ("Start", MouseAction::Start),
        ("Pause", MouseAction::Pause),
        ("Reset", MouseAction::Reset),
        ("Skip", MouseAction::Skip),
        ("Theme", MouseAction::Theme),
        ("Help", MouseAction::Help),
    ];
    for (index, (label, action)) in buttons.iter().enumerate() {
        let fg = if *label == "Skip" {
            theme.danger
        } else {
            theme.primary
        };
        frame.render_widget(
            Paragraph::new(*label)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(theme.border)),
                )
                .style(Style::default().fg(fg).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            chunks[index],
        );
        state.mouse_zones_mut().add(chunks[index], *action);
    }
    let help = format!(
        "Space pause/resume  s start  n skip  r reset  t theme  a font  q quit  {}",
        format_duration(snapshot.remaining)
    );
    frame.render_widget(
        Paragraph::new(help).style(Style::default().fg(theme.muted)),
        chunks[6],
    );
}

fn render_panel(frame: &mut Frame, state: &UiState, area: Rect, title: &str, text: &str) {
    let theme = state.theme();
    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            )
            .style(Style::default().fg(theme.foreground))
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn render_help(frame: &mut Frame, area: Rect) {
    let text = "Space pause/resume\ns start\nr reset\nn next phase\nq quit\n? help\nt theme\np profile (CLI)\nm tab/mode\na ASCII font\n+ add minute\n- remove minute\nTab next panel\nMouse: click buttons and tabs";
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn format_duration(duration: std::time::Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    format!("{:02}:{:02}", minutes, seconds % 60)
}
