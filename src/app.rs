use color_eyre::eyre::WrapErr;

use crate::tui;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Alignment, Stylize},
    symbols::border,
    terminal::Frame,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    should_exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<u8> {
        while !self.should_exit {
            // Draw all the widgets
            terminal.draw(|frame| self.render_frame(frame))?;

            // Handle events
            self.handle_events().wrap_err("handle_events failed")?;
        }

        Ok(self.counter)
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
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }

    fn increment_counter(&mut self) -> color_eyre::Result<()> {
        let new_value = self
            .counter
            .checked_add(1)
            .ok_or(format!("Overflow when adding 1 from {}", self.counter))
            .map_err(color_eyre::eyre::Error::msg)?;

        self.counter = new_value;
        Ok(())
    }

    fn decrement_counter(&mut self) -> color_eyre::Result<()> {
        let new_value = self
            .counter
            .checked_sub(1)
            .ok_or(format!(
                "Underflow when subtracting 1 from {}",
                self.counter
            ))
            .map_err(color_eyre::eyre::Error::msg)?;

        self.counter = new_value;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold().into(),
            " Increment ".into(),
            " <Right>".blue().bold().into(),
            " Quit ".into(),
            " <Q> ".blue().bold().into(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
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
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);
    }

    #[test]
    fn test_handle_exit() -> io::Result<()> {
        // If a user presses 'q', we should quit
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.should_exit, true);

        Ok(())
    }
}
