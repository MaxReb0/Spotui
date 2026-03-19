use ratatui::{
    Frame,
    layout::Rect,
    prelude::Widget,
    widgets::{Block, Paragraph},
};

use crate::app::app::App;

pub fn render(area: Rect, frame: &mut Frame, app: &App) {
    let welcome = format!("Hello, {}!", app.username().unwrap_or("Unknown"));
    Paragraph::new(welcome)
        .centered()
        .block(Block::bordered().title(" Home "))
        .render(area, frame.buffer_mut());
}
