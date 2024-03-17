use crate::app::{App, Cell};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

// Main theme color
const THEME_COLOR: Color = Color::LightGreen;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(frame.size());

    let universe_block = Block::bordered()
        .title(" Game of Life ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    // Build universe widget
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

    // Build commands widget
    let command_block = Block::default();
    let command_widget = Paragraph::new("[space] next generation, [r]estart, [q]uit")
        .style(Style::default().fg(THEME_COLOR))
        .block(command_block);

    // Build widgets
    frame.render_widget(universe_widget, layout[0]);
    frame.render_widget(command_widget, layout[1]);
}
