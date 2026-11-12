use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;

const BG: Color = Color::Black;
const BLUE: Color = Color::Rgb(0, 100, 255);
const LIGHT_BLUE: Color = Color::Rgb(100, 180, 255);
const SEL_BG: Color = Color::Rgb(15, 30, 70);
const DONE_COLOR: Color = Color::Rgb(80, 220, 160);
const PENDING_COLOR: Color = Color::Yellow;

pub fn draw(f: &mut Frame, app: &App, list_state: &mut ListState) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(BG)), area);

    let input_height = if app.input_mode { 3 } else { 0 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(input_height),
            Constraint::Length(1),
        ])
        .split(area);

    draw_list(f, app, list_state, chunks[0]);
    if app.input_mode {
        draw_input(f, app, chunks[1]);
    }
    draw_status(f, app, chunks[2]);
}

fn draw_list(f: &mut Frame, app: &App, list_state: &mut ListState, area: Rect) {
    let done_count = app.tasks.iter().filter(|t| t.done).count();
    let title = format!(" Tasks ({}/{}) ", done_count, app.tasks.len());
    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BLUE))
        .style(Style::default().bg(BG).fg(Color::White));

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let selected = i == app.selected_index;
            let row_bg = if selected { SEL_BG } else { BG };

            let text_style = if selected {
                Style::default()
                    .fg(LIGHT_BLUE)
                    .bg(row_bg)
                    .add_modifier(Modifier::BOLD)
            } else if task.done {
                Style::default().fg(Color::Gray).bg(row_bg)
            } else {
                Style::default().fg(Color::White).bg(row_bg)
            };

            let (bracket, bracket_color) = if task.done {
                ("[*]", DONE_COLOR)
            } else {
                ("[ ]", PENDING_COLOR)
            };

            let prefix = if selected { "▶ " } else { "  " };

            // براکت‌ها همیشه بعد از متن تسک در انتهای خط می‌آیند
            let line = Line::from(vec![
                Span::styled(prefix, Style::default().fg(LIGHT_BLUE).bg(row_bg)),
                Span::styled(task.text.as_str(), text_style),
                Span::styled(" ", Style::default().bg(row_bg)),
                Span::styled(
                    bracket,
                    Style::default()
                        .fg(bracket_color)
                        .bg(row_bg)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .style(Style::default().bg(row_bg));
            ListItem::new(line)
        })
        .collect();

    if app.tasks.is_empty() {
        let empty = Paragraph::new(Span::styled(
            "No tasks yet. Press 'a' to add one.",
            Style::default().fg(Color::Gray),
        ))
        .block(block);
        f.render_widget(empty, area);
        return;
    }

    list_state.select(Some(app.selected_index));
    f.render_stateful_widget(List::new(items).block(block), area, list_state);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " New task ",
            Style::default().fg(LIGHT_BLUE).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(LIGHT_BLUE))
        .style(Style::default().bg(BG));
    let p = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(Color::White).bg(BG))
        .block(block);
    f.render_widget(p, area);

    let max_x = area.x + area.width.saturating_sub(2);
    let x = (area.x + 1 + app.input_buffer.chars().count() as u16).min(max_x);
    f.set_cursor_position((x, area.y + 1));
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let (text, color) = if let Some(err) = &app.error {
        (err.as_str(), Color::Red)
    } else if app.input_mode {
        ("Type task text, Enter to save, Esc to cancel", Color::Gray)
    } else {
        (
            "Up/Down: Move | Tab: Toggle | a: Add | d: Delete | q: Quit",
            Color::Gray,
        )
    };
    let p = Paragraph::new(text).style(Style::default().fg(color).bg(BG));
    f.render_widget(p, area);
}
