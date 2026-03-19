use ratatui::{
    Frame,
    layout::Rect,
    prelude::Widget,
    widgets::{Block, Paragraph},
};

use crate::app::app::App;

pub fn render(area: Rect, frame: &mut Frame, _app: &App) {
    let welcome = String::from("TBD!");
    Paragraph::new(welcome)
        .centered()
        .block(Block::bordered().title(" Playlists "))
        .render(area, frame.buffer_mut());
}
