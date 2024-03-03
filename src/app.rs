use color_eyre::eyre::WrapErr;

use crate::tui;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Alignment, Stylize},
    symbols::border,
    terminal::Frame,
    text::{Line, Text},
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Default)]
pub struct App {
    search_text: String,
    should_exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<String> {
        while !self.should_exit {
            // Draw all the widgets
            terminal.draw(|frame| self.render_frame(frame))?;

            // Handle events
            self.handle_events().wrap_err("handle_events failed")?;
        }

        Ok(self.search_text.clone())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // It's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}"))?,
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('Q') => self.exit(),
            KeyCode::Backspace => {
                self.search_text.pop();
            }
            KeyCode::Char(char) => {
                self.search_text.push(char);
            }
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Fuzzy search sample ".bold());

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Search text: ".into(),
            self.search_text.clone().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_handle_counter_interaction() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('a').into()).unwrap();
        assert_eq!(app.search_text, String::from("a"));

        app.handle_key_event(KeyCode::Char('b').into()).unwrap();
        assert_eq!(app.search_text, String::from("ab"));
    }

    #[test]
    fn test_handle_exit() -> color_eyre::Result<()> {
        // If a user presses 'q', we should quit
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert_eq!(app.should_exit, true);

        Ok(())
    }
}
