use super::component::Component;
use crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct Input {
    value: String,
}

impl Input {
    pub fn new() -> Self {
        Input {
            value: "dupa input".to_string(),
        }
    }
}

impl Component for Input {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(self.value.as_str()).block(block);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, event: Event) {
        // No event handling needed for now
    }
}
