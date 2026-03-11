use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::Widget,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::app::{App, InputMode};

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    let [help_area, search_bar, results] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .areas(area);

    //TODO: Render the help page to show tips for navigating.
    //
    //

    Paragraph::new(app.search_query().as_str())
        .style(match app.current_input_mode() {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"))
        .render(search_bar, buf);
}
