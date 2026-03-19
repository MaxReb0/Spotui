use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    prelude::Widget,
    style::{Color, Style},
    widgets::{Block, List, Paragraph},
};
use rspotify::model::{SearchResult, SearchType};

use crate::app::app::{App, InputMode};

pub fn render(area: Rect, frame: &mut Frame, app: &App) {
    let [help_area, search_bar, results] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .areas(area);

    //TODO: Render the help page to show tips for navigating.
    //
    //

    // Search Bar.
    Paragraph::new(app.search_query().as_str())
        .style(match app.current_input_mode() {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"))
        .render(search_bar, frame.buffer_mut());

    // TODO: Investigate why the cursor is not blinking.
    match app.current_input_mode() {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor_position(Position::new(
            search_bar.x + app.character_index() as u16 + 1,
            search_bar.y + 1,
        )),
    }

    // Now draw the results page.
    match app.active_search_type() {
        SearchType::Track => {
            if let Some(data) = app.search_results().tracks {
                let names: Vec<String> = data.items.iter().map(|t| t.name.clone()).collect();
                let list = List::new(names).block(Block::bordered().title("Results"));
                frame.render_widget(list, results);
            } else {
                Paragraph::new("")
                    .centered()
                    .block(Block::bordered().title("Results"))
                    .render(results, frame.buffer_mut());
            }
        }
        SearchType::Album => {
            println!("Whoa")
        }
        _ => {}
    }
}
