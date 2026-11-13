use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;

const BG: Color = Color::Black;
const ACCENT: Color = Color::Rgb(90, 210, 170);
const MUTED: Color = Color::Gray;
const DONE_COLOR: Color = Color::Rgb(90, 210, 170);
const PENDING_COLOR: Color = Color::Rgb(220, 190, 110);

pub fn draw(f: &mut Frame, app: &App, list_state: &mut ListState) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(BG)), area);

    let input_height = if app.input_mode { 2 } else { 0 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(input_height),
            Constraint::Length(1),
        ])
        .split(area);

    draw_header(f, app, chunks[0]);
    draw_list(f, app, list_state, chunks[1]);
    if app.input_mode {
        draw_input(f, app, chunks[2]);
    }
    draw_status(f, app, chunks[3]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let done = app.tasks.iter().filter(|task| task.done).count();
    let total = app.tasks.len();
    let title = Line::from(vec![
        Span::styled(
            "  TASKS",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  /  ", Style::default().fg(MUTED)),
        Span::styled(
            format!("{done} completed"),
            Style::default().fg(ACCENT),
        ),
        Span::styled(
            format!(" of {total}"),
            Style::default().fg(MUTED),
        ),
    ]);
    f.render_widget(Paragraph::new(title), area);
}

fn draw_list(f: &mut Frame, app: &App, list_state: &mut ListState, area: Rect) {
    if app.tasks.is_empty() {
        list_state.select(None);
        let empty = Paragraph::new(Line::from(vec![
            Span::styled("  Nothing on the list yet. ", Style::default().fg(MUTED)),
            Span::styled("Press a", Style::default().fg(ACCENT)),
            Span::styled(" to add your first task.", Style::default().fg(MUTED)),
        ]));
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let selected = index == app.selected_index;
            let (check, check_color) = if task.done {
                ("[✓]", DONE_COLOR)
            } else {
                ("[ ]", PENDING_COLOR)
            };

            let marker = if selected { "  › " } else { "    " };
            let text_style = if task.done {
                Style::default().fg(MUTED)
            } else if selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    marker,
                    Style::default().fg(if selected { ACCENT } else { MUTED }),
                ),
                Span::styled(task.text.as_str(), text_style),
                Span::raw("  "),
                Span::styled(
                    check,
                    Style::default()
                        .fg(check_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]))
        })
        .collect();

    list_state.select(Some(app.selected_index));
    f.render_stateful_widget(List::new(items), area, list_state);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled("  +  ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(app.input_buffer.as_str(), Style::default().fg(Color::White)),
    ]);
    f.render_widget(Paragraph::new(line), area);

    let text_width = UnicodeWidthStr::width(app.input_buffer.as_str());
    let cursor_offset = u16::try_from(text_width).unwrap_or(u16::MAX);
    let max_x = area.x + area.width.saturating_sub(1);
    let x = area
        .x
        .saturating_add(5)
        .saturating_add(cursor_offset)
        .min(max_x);
    f.set_cursor_position((x, area.y));
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let (text, color) = match (&app.error, app.input_mode) {
        (Some(error), true) => (
            format!("Enter: save  ·  Esc: cancel  ·  {error}"),
            Color::Red,
        ),
        (Some(error), false) => (error.clone(), Color::Red),
        (None, true) => (
            "Enter: save    Esc: cancel".to_string(),
            MUTED,
        ),
        (None, false) => (
            "↑/↓ navigate   Tab complete   a add   d delete   q quit".to_string(),
            MUTED,
        ),
    };
    f.render_widget(
        Paragraph::new(format!("  {text}")).style(Style::default().fg(color).bg(BG)),
        area,
    );
}
