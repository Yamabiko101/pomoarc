use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::domain::timer::TimerSnapshot;

use super::{animation, ascii_font, mouse::MouseAction, InputMode, Tab, UiState};

pub fn render(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot) {
    let area = frame.area();
    let theme = state.theme();
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().style(Style::default().bg(theme.background).fg(theme.foreground)),
        area,
    );

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(area);

    state.mouse_zones_mut().clear();
    render_header(frame, state, root[0]);
    render_body(frame, state, snapshot, root[1]);
    render_footer(frame, state, snapshot, root[2]);

    if state.show_help() {
        render_help(frame, centered_rect(70, 62, area));
    }
    if state.input_mode().is_some() {
        render_input_modal(frame, state, centered_rect(64, 34, area));
    }

    let _ = theme;
}

fn render_header(frame: &mut Frame, state: &mut UiState, area: Rect) {
    let theme = state.theme();
    let titles = ["Timer", "Tasks", "Notes", "Stats", "Settings"]
        .into_iter()
        .map(|title| Line::from(format!("  {title}  ")))
        .collect::<Vec<_>>();
    let selected = match state.tab() {
        Tab::Timer => 0,
        Tab::Tasks => 1,
        Tab::Notes => 2,
        Tab::Stats => 3,
        Tab::Settings => 4,
    };
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title("pomoarc")
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(theme.border))
                .style(Style::default().bg(theme.background)),
        )
        .select(selected)
        .style(Style::default().fg(theme.muted).bg(theme.background))
        .highlight_style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs, area);

    let tab_width = area.width / 5;
    state.mouse_zones_mut().add(
        Rect::new(area.x, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Timer),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Tasks),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width * 2, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Notes),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width * 3, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Stats),
    );
    state.mouse_zones_mut().add(
        Rect::new(area.x + tab_width * 4, area.y, tab_width, area.height),
        MouseAction::Tab(Tab::Settings),
    );
}

fn render_body(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot, area: Rect) {
    match state.tab() {
        Tab::Timer => render_timer(frame, state, snapshot, area),
        Tab::Tasks => render_tasks(frame, state, area),
        Tab::Notes => render_notes(frame, state, area),
        Tab::Stats => render_panel(frame, state, area, "Stats", "Run `pomoarc stats --json` for persisted stats.\nToday, streak, best hour and tag totals are stored locally."),
        Tab::Settings => render_panel(frame, state, area, "Settings", "Config lives at `pomoarc config path`.\nHotkeys: t theme, a font, m tab, ? help.\nNotes: `pomoarc note add/list/edit/delete`."),
    }
}

fn render_timer(frame: &mut Frame, state: &mut UiState, snapshot: &TimerSnapshot, area: Rect) {
    let theme = state.theme();
    let left_width = if area.width >= 120 { 78 } else { 70 };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(left_width),
            Constraint::Percentage(100 - left_width),
        ])
        .split(area);
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(if area.height >= 28 { 13 } else { 9 }),
            Constraint::Length(3),
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
                .border_style(Style::default().fg(theme.primary))
                .style(Style::default().bg(theme.background)),
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
                .border_style(Style::default().fg(theme.progress_empty))
                .style(Style::default().bg(theme.background)),
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
                    .border_style(Style::default().fg(theme.border))
                    .style(Style::default().bg(theme.background)),
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
    let mut text = vec![
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
            "Companion",
            Style::default()
                .fg(theme.secondary)
                .add_modifier(Modifier::BOLD),
        ),
    ];
    text.extend(companion_lines(
        snapshot.phase.label(),
        state.frame(),
        &theme,
    ));
    text.push(Line::raw(""));
    text.push(Line::from(vec![
        Span::styled("Hint: ", Style::default().fg(theme.warning)),
        Span::raw("Press ? for controls"),
    ]));
    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title("pomoarc")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border))
                    .style(Style::default().bg(theme.background)),
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
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(9),
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
                        .border_style(Style::default().fg(theme.border))
                        .style(Style::default().bg(theme.background)),
                )
                .style(Style::default().fg(fg).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            chunks[index],
        );
        state.mouse_zones_mut().add(chunks[index], *action);
    }
    let help = format!(
        "Space pause/resume | s start | n skip | t theme | a font | ? help | q quit | {}",
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
                    .border_style(Style::default().fg(theme.border))
                    .style(Style::default().bg(theme.background)),
            )
            .style(Style::default().fg(theme.foreground))
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn render_notes(frame: &mut Frame, state: &mut UiState, area: Rect) {
    let theme = state.theme();
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);
    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(14),
            Constraint::Length(14),
            Constraint::Length(14),
            Constraint::Min(10),
        ])
        .split(sections[0]);
    let button_specs = [
        ("Add note", MouseAction::AddNote, theme.accent),
        ("Edit latest", MouseAction::EditNote, theme.warning),
        ("Delete", MouseAction::DeleteNote, theme.danger),
    ];
    for (index, (label, action, fg)) in button_specs.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(*label)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(*fg))
                        .style(Style::default().bg(theme.background)),
                )
                .style(Style::default().fg(*fg).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            buttons[index],
        );
        state.mouse_zones_mut().add(buttons[index], *action);
    }
    let mut lines = vec![
        Line::styled(
            "Notes",
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(vec![
            Span::styled("add ", Style::default().fg(theme.accent)),
            Span::raw("click Add note or press i"),
        ]),
        Line::from(vec![
            Span::styled("edit ", Style::default().fg(theme.warning)),
            Span::raw("click Edit latest or press e"),
        ]),
        Line::from(vec![
            Span::styled("delete ", Style::default().fg(theme.danger)),
            Span::raw("click Delete or press x"),
        ]),
        Line::raw(""),
    ];

    if state.notes().is_empty() {
        lines.push(Line::styled(
            "No notes yet. Capture one from the CLI and reopen the TUI.",
            Style::default().fg(theme.muted),
        ));
    } else {
        for note in state.notes() {
            lines.push(Line::from(vec![
                Span::styled("▸ ", Style::default().fg(theme.accent)),
                Span::styled(format!("#{} ", note.id), Style::default().fg(theme.muted)),
                Span::raw(note.body.as_str()),
            ]));
        }
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title("Notes")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.primary))
                    .style(Style::default().bg(theme.background)),
            )
            .style(Style::default().fg(theme.foreground))
            .wrap(Wrap { trim: true }),
        sections[1],
    );
}

fn render_tasks(frame: &mut Frame, state: &mut UiState, area: Rect) {
    let theme = state.theme();
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);
    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(14), Constraint::Min(10)])
        .split(sections[0]);
    frame.render_widget(
        Paragraph::new("Add task")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.accent))
                    .style(Style::default().bg(theme.background)),
            )
            .style(
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        buttons[0],
    );
    state
        .mouse_zones_mut()
        .add(buttons[0], MouseAction::AddTask);

    let mut lines = vec![
        Line::styled(
            "Tasks",
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(vec![
            Span::styled("active ", Style::default().fg(theme.warning)),
            Span::raw(state.task().to_string()),
        ]),
        Line::from(vec![
            Span::styled("add ", Style::default().fg(theme.accent)),
            Span::raw("click Add task or press i"),
        ]),
        Line::raw(""),
    ];

    if state.tasks().is_empty() {
        lines.push(Line::styled(
            "No tasks yet. Add one without leaving the TUI.",
            Style::default().fg(theme.muted),
        ));
    } else {
        let tasks = state.tasks().to_vec();
        for (index, task) in tasks.iter().enumerate() {
            lines.push(Line::from(vec![
                Span::styled("▸ ", Style::default().fg(theme.accent)),
                Span::raw(task.title.clone()),
                Span::styled(
                    format!("  ({})", task.tag),
                    Style::default().fg(theme.muted),
                ),
            ]));
            let row_y = sections[1].y + 5 + index as u16;
            if row_y < sections[1].y.saturating_add(sections[1].height) {
                state.mouse_zones_mut().add(
                    Rect::new(sections[1].x, row_y, sections[1].width, 1),
                    MouseAction::SelectTask(index),
                );
            }
        }
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title("Tasks")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.primary))
                    .style(Style::default().bg(theme.background)),
            )
            .style(Style::default().fg(theme.foreground))
            .wrap(Wrap { trim: true }),
        sections[1],
    );
}

fn render_help(frame: &mut Frame, area: Rect) {
    let text = "Space pause/resume\ns start\nr reset\nn next phase\nq quit\n? help\nt theme\nm tab/mode\na ASCII font\ni add task/note in current tab\ne edit latest note\nx delete latest note\n+ add minute\n- remove minute\nTab next panel\nMouse: click buttons, tabs, and task rows";
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_input_modal(frame: &mut Frame, state: &UiState, area: Rect) {
    let theme = state.theme();
    let (title, hint) = match state.input_mode() {
        Some(InputMode::Task) => ("New task", "Type a task, Enter to save, Esc to cancel"),
        Some(InputMode::Note) => ("New note", "Type a note, Enter to save, Esc to cancel"),
        Some(InputMode::EditNote(_)) => ("Edit note", "Edit text, Enter to save, Esc to cancel"),
        None => return,
    };
    frame.render_widget(Clear, area);
    let lines = vec![
        Line::styled(
            hint,
            Style::default()
                .fg(theme.muted)
                .add_modifier(Modifier::ITALIC),
        ),
        Line::raw(""),
        Line::from(vec![
            Span::styled("> ", Style::default().fg(theme.accent)),
            Span::raw(state.input_buffer()),
            Span::styled("█", Style::default().fg(theme.primary)),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.primary))
                    .style(Style::default().bg(theme.background)),
            )
            .style(Style::default().fg(theme.foreground).bg(theme.background))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn companion_lines<'a>(phase: &str, frame: u64, theme: &super::theme::Theme) -> Vec<Line<'a>> {
    let blink = frame.is_multiple_of(4);
    let tail = if frame.is_multiple_of(2) { "~" } else { "^" };
    let toy = if frame.is_multiple_of(3) { "◆" } else { "·" };
    let (eye_l, eye_r, label, accent) = match phase {
        "Paused" => ("-", "-", "resting", theme.muted),
        "Short break" | "Long break" => ("o", "o", "break", theme.warning),
        _ if blink => ("-", "-", "focus", theme.accent),
        _ => ("•", "•", "focus", theme.accent),
    };

    vec![
        Line::from(vec![
            Span::styled("  /\\_/\\     ", Style::default().fg(theme.secondary)),
            Span::styled(toy, Style::default().fg(theme.danger)),
        ]),
        Line::from(vec![
            Span::styled(" ( ", Style::default().fg(theme.secondary)),
            Span::styled(
                eye_l,
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled(".", Style::default().fg(theme.secondary)),
            Span::styled(
                eye_r,
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" )  ", Style::default().fg(theme.secondary)),
            Span::styled(label, Style::default().fg(accent)),
        ]),
        Line::from(vec![
            Span::styled(" / > ", Style::default().fg(theme.secondary)),
            Span::styled("tomato", Style::default().fg(theme.danger)),
        ]),
        Line::from(vec![
            Span::styled(" tail ", Style::default().fg(theme.muted)),
            Span::styled(tail, Style::default().fg(theme.primary)),
        ]),
    ]
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
