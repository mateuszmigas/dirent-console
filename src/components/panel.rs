use super::component::Component;
use crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct Panel {
    title: Option<String>,
    content: String,
}

impl Panel {
    pub fn new(area: Rect) -> Self {
        Panel {
            title: None,
            content: String::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }
}

impl Component for Panel {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.clone().unwrap_or_default());

        let paragraph = Paragraph::new(self.content.as_str()).block(block);

        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, event: Event) {
        // No event handling needed for now
    }
}
