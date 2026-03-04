use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!();
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        todo!();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
