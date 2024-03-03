use color_eyre::eyre::WrapErr;
use ratatui::style::Color;
use ratatui::style::Styled;

use crate::backend;
use crate::tui;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::{Alignment, Stylize},
    symbols::border,
    terminal::Frame,
    text::{Line, Span, Text},
    widgets::List,
    widgets::ListItem,
    widgets::StatefulWidget,
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Default)]
pub struct App {
    search_text: String,
    current_project: Option<String>,
    current_sequence: Option<String>,
    should_exit: bool,
    search_items: Vec<String>,
    highlighted_item_index: usize,
}

impl App {
    pub fn new() -> Self {
        let projects = backend::get_projects();

        App {
            search_text: String::new(),
            current_project: None,
            current_sequence: None,
            should_exit: false,
            search_items: projects,
            highlighted_item_index: 0,
        }
    }

    fn search(&mut self) -> Vec<String> {
        // TODO: This search should happen hierarchically
        // e.g.: If the project has been chosen,
        // choose the sequence/asset, if the sequence/asset has been chosen,
        // choose the shot, etc..
        let all_items = backend::get_projects();

        // Keep track of the current highlightem item
        let current_highlighted_index = self.highlighted_item_index;
        let current_highlighted_item = self
            .search_items
            .iter()
            .enumerate()
            .find(|(i, _item)| i == &current_highlighted_index)
            .map(|(_i, item)| item);

        // TODO: Proper fuzzy finding instead of just 'contains'
        let new_items: Vec<_> = all_items
            .iter()
            .filter(|i| i.contains(&self.search_text))
            .map(|i| i.to_string())
            .collect();

        // Restore the highlighted element, if possible
        if let Some(item) = current_highlighted_item {
            let new_highlighted_index = match new_items.binary_search(&item) {
                Ok(s) => s,
                Err(_) => 0,
            };
            self.highlighted_item_index = new_highlighted_index;
        }

        new_items
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<String> {
        while !self.should_exit {
            // Draw all the widgets
            terminal.draw(|frame| self.render_frame(frame))?;

            // Handle events
            self.handle_events().wrap_err("handle_events failed")?;
        }

        Ok(self.search_text.clone())
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
            KeyCode::Char('Q') => self.exit(),
            KeyCode::Backspace => {
                // If the search is empty show all projects
                self.search_text.pop();
                if self.search_text.is_empty() {
                    // FIXME: This will depend on the current 'state' of the app
                    // e.g.: if we are searching for sequences, add sequences,
                    // if we are searching for shots, show all shots, etc..
                    self.search_items = backend::get_projects();
                } else {
                    self.search_items = self.search();
                }
            }
            KeyCode::Char(char) => {
                self.search_text.push(char);
                self.search_items = self.search();
            }
            KeyCode::Tab | KeyCode::Down => {
                let next_index = self
                    .highlighted_item_index
                    .saturating_add(1)
                    .min(self.search_items.len() - 1);

                self.highlighted_item_index = next_index;
            }
            KeyCode::BackTab | KeyCode::Up => {
                let next_index = self.highlighted_item_index.saturating_sub(1).max(0);
                self.highlighted_item_index = next_index;
            }
            KeyCode::Enter => {
                // TODO: Select an item and go to the next stage
            }
            _ => {}
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(" Fuzzy search sample (press shift+q to quit) ")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_search_area(&self, area: Rect, buf: &mut Buffer) {
        let counter_text = Text::from(vec![Line::from(vec![
            "Search text: ".into(),
            self.search_text.clone().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_search_items(&self, area: Rect, buf: &mut Buffer) {
        let inner_block = Block::default().borders(Borders::NONE).white();

        let items: Vec<ListItem> = self
            .search_items
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Span::from(Span::raw(format!("{i}: {m}")));
                if i == self.highlighted_item_index {
                    ListItem::new(content).set_style(Color::Magenta)
                } else {
                    ListItem::new(content)
                }
            })
            .collect();

        let items = List::new(items).block(inner_block.title("> Results").italic());
        Widget::render(items, area, buf);
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a space for header, search text and the search items.
        let vertical_layout = Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ]);

        // What is this magic ?
        let [header_area, search_area, items_area] = vertical_layout.areas(area);

        self.render_header(header_area, buf);
        self.render_search_area(search_area, buf);
        self.render_search_items(items_area, buf);
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
