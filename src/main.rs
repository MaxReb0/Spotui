use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, Frame, style::Stylize, text::Line};

mod app;
use app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // let mut terminal = ratatui::init();
    ratatui::run(|terminal| App::default().run(terminal))?;
    // ratatui::run(|terminal| );
    ratatui::restore();
    Ok(())
}

fn render(frame: &mut Frame) {
    let title = Line::from(" Hello, this is actually working! ".bold());
    let block = ratatui::widgets::Block::bordered()
        .title(title.centered())
        .border_type(ratatui::widgets::BorderType::HeavyDoubleDashed)
        .title("TESTING");

    frame.render_widget(block, frame.area());
}
