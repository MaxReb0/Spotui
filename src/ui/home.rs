use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Widget,
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    let welcome = format!("Hello, {}!", app.username().unwrap_or("Unknown"));
    Paragraph::new(welcome)
        .centered()
        .block(Block::bordered().title(" Home "))
        .render(area, buf);
}
