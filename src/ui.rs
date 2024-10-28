use crate::app::{App, Cell};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

// Main theme color
const THEME_COLOR: Color = Color::Cyan;

pub fn ui(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(frame.size());

    // Render widgets
    render_universe_widget(app, frame, layout[0]);
    render_command_widget(frame, layout[1]);
}

fn render_universe_widget(app: &mut App, frame: &mut Frame, area: Rect) {
    let universe_block = Block::bordered()
        .title(" Game of Life ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    let universe = &app.universe;
    let mut lines = vec![];
    for row in 0..universe.height {
        let mut spans = vec![];
        for col in 0..universe.width {
            let idx = universe.get_cell_idx(row, col);
            let cell_style = match universe.cells[idx] {
                Cell::Alive => Style::default().bg(THEME_COLOR),
                Cell::Dead => Style::default(),
            };
            let cell_span = Span::styled("  ", cell_style);
            spans.push(cell_span);
        }
        let line = Line::from(spans);
        lines.push(line);
    }
    let universe_widget = Paragraph::new(lines).block(universe_block);
    frame.render_widget(universe_widget, area);
}

fn render_command_widget(frame: &mut Frame, area: Rect) {
    let command_block = Block::default();
    let command_widget = Paragraph::new("[space] next generation, [r]estart, [q]uit")
        .style(Style::default().fg(THEME_COLOR))
        .block(command_block);
    frame.render_widget(command_widget, area);
}
