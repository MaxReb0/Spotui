use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    prelude::Widget,
    style::{Color, Style},
    symbols,
    widgets::{Block, Paragraph, Row, Table, Tabs},
};
use strum::IntoEnumIterator;

use crate::app::{
    app::{App, InputMode},
    search_tab::SearchTab,
};

pub fn render(area: Rect, frame: &mut Frame, app: &mut App) {
    let [tabs_area, search_bar, results_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .areas(area);

    //TODO: Add proper representation of navigation messages!

    let titles = SearchTab::iter().map(|t| t.to_string());
    let selected = SearchTab::iter()
        .position(|t| t.search_type() == app.active_search_type())
        .unwrap_or(0);

    Tabs::new(titles)
        .select(selected)
        .style(Style::default().fg(Color::Cyan))
        .divider(symbols::DOT)
        .render(tabs_area, frame.buffer_mut());

    // Search Bar implementation. It needs to be displayed differently based off the state you are
    // in
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
    if let Some(results) = app.search_results().active(app.active_search_type()) {
        let rows: Vec<Row> = (0..results.len()).map(|i| results.row_at(i)).collect();
        let title = format!(
            "Results ({}/{})",
            app.table_state_mut().selected().unwrap_or(0),
            results.len(),
        );
        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .header(results.headers())
        .block(Block::bordered().title(title))
        .row_highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(table, results_area, app.table_state_mut());
    } else {
        Paragraph::new("")
            .centered()
            .block(Block::bordered().title("Results"))
            .render(results_area, frame.buffer_mut());
    }
}
