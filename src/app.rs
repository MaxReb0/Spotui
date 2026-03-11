use std::{sync::Arc, time::Duration};

use crate::spotify::spotify_event::SpotifyEvent;
use crate::ui;
use crate::{spotify::client::SpotifyApi, trace_dbg};
use color_eyre::eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use itertools::Itertools;
use ratatui::layout::Layout;
use ratatui::style::{Color, Style};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::Tabs;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::Widget,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;

#[derive(Debug, Display, EnumIter, Clone, Copy)]
enum View {
    Home,
    Search,
    Playlists,
    //TODO: Add more to this as new pages are added.
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    spotify_client: Arc<dyn SpotifyApi>,
    current_view: View,
}

impl App {
    pub fn new(spotify_client: Arc<dyn SpotifyApi>) -> Self {
        App {
            exit: false,
            spotify_client,
            current_view: View::Home,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        // TODO: We need to actually add the threading for spawning and consuming Spotify API
        // calls.
        let (tx, mut rx) = tokio::sync::mpsc::channel::<SpotifyEvent>(32);
        let mut interval = tokio::time::interval(Duration::from_secs_f32(1.0 / 60.0));
        let mut events = EventStream::new();

        while !self.exit {
            tokio::select! {
                _ = interval.tick() => {terminal.draw(|frame| self.draw(frame))?;},
                Some(Ok(event)) = events.next() => {self.handle_events(event, &tx).await?;}
                Some(_) = rx.recv() => {}
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn username(&self) -> Option<&str> {
        self.spotify_client.username()
    }

    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
        // TODO: Clean up UI
        let keys = [
            ("H/←", "Left"),
            ("L/→", "Right"),
            ("K/↑", "Up"),
            ("J/↓", "Down"),
            ("Q/(Ctrl + C)", "Quit"),
        ];
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {key} "), Color::Green);
                let desc = Span::styled(format!(" {desc} "), Color::Green);
                [key, desc]
            })
            .collect_vec();

        Line::from(spans)
            .centered()
            .style((Color::Indexed(236), Color::Indexed(232)))
            .render(area, buf);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn change_view(&mut self, view: View) {
        self.current_view = view;
    }

    async fn handle_events(&mut self, event: Event, tx: &Sender<SpotifyEvent>) -> Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, tx).await?;
            }
            _ => {}
        };
        Ok(())
    }

    async fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        tx: &Sender<SpotifyEvent>,
    ) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::Enter => {
                let tx = tx.clone();
                let spotify_client = Arc::clone(&self.spotify_client);
                tokio::spawn(async move {
                    match spotify_client.current_playback().await {
                        Ok(data) => {
                            tx.send(SpotifyEvent::CurrentlyPlaying(data)).await.ok();
                        }
                        Err(e) => {
                            trace_dbg!(level: tracing::Level::ERROR, e.to_string());
                        }
                    }
                });
            }
            KeyCode::Char('1') => self.change_view(View::Home),
            KeyCode::Char('2') => self.change_view(View::Search),
            KeyCode::Char('3') => self.change_view(View::Playlists),
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

        let titles = View::iter().map(|v| v.to_string());
        let selected = self.current_view as usize;

        Tabs::new(titles)
            .select(selected)
            .style(Style::default().green())
            .divider(symbols::DOT)
            .render(layout[0], buf);

        match self.current_view {
            View::Home => {
                ui::home::render(layout[1], buf, self);
            }
            View::Search => {
                ui::search::render(layout[1], buf, self);
            }
            View::Playlists => {
                ui::playlists::render(layout[1], buf, self);
            }
        }
        App::render_bottom_bar(layout[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use color_eyre::eyre::Ok;
    use rspotify::model::CurrentlyPlayingContext;

    #[derive(Debug)]
    struct MockSpotifyApi {
        username: Option<String>,
    }

    #[async_trait]
    impl SpotifyApi for MockSpotifyApi {
        fn username(&self) -> Option<&str> {
            self.username.as_deref()
        }

        async fn current_playback(&self) -> Result<Option<CurrentlyPlayingContext>> {
            Ok(None)
        }
    }

    fn make_app(username: Option<&str>) -> App {
        App::new(Arc::new(MockSpotifyApi {
            username: username.map(String::from),
        }))
    }

    #[tokio::test]
    async fn pressing_q_sets_exit() {
        let mut app = make_app(Some("Test User"));
        assert!(!app.exit);
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        app.handle_key_event(key, &tx).await.unwrap();
        assert!(app.exit)
    }

    #[tokio::test]
    async fn pressing_ctrl_c_sets_exit() {
        let mut app = make_app(Some("Test User"));
        assert!(!app.exit);
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        app.handle_key_event(key, &tx).await.unwrap();
        assert!(app.exit)
    }

    #[tokio::test]
    async fn enter_sends_spotify_event() {
        let mut app = make_app(Some("Test User"));
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        app.handle_key_event(key, &tx).await.unwrap();
        let result = rx.recv().await.unwrap();
        assert_eq!(result, SpotifyEvent::CurrentlyPlaying(None))
    }

    #[tokio::test]
    async fn test_handle_event_ok() {
        let mut app = make_app(Some("Test User"));
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        let result = app.handle_events(event, &tx).await;
        assert_eq!(result.unwrap(), ());
        assert!(app.exit)
    }
}
