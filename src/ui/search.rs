use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Widget,
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render(area: Rect, buf: &mut Buffer, _app: &App) {
    let welcome = String::from("TBD!");
    Paragraph::new(welcome)
        .centered()
        .block(Block::bordered().title(" Search "))
        .render(area, buf);
}
