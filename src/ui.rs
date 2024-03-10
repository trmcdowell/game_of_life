use crate::app::App;
use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};

// Main theme color
const THEME_COLOR: Color = Color::LightGreen;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_block = Block::bordered()
        .title(" Game of Life ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    let universe_widget = Paragraph::new(app.universe.to_string()).block(main_block);

    frame.render_widget(universe_widget, frame.size());
}
