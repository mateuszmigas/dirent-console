use super::{
    component::{Component, Node},
    RenderingContext,
};
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
    fn render(&self, ctx: &mut RenderingContext, area: Rect) -> Vec<Node> {
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(self.value.as_str()).block(block);
        ctx.frame.render_widget(paragraph, area);
        vec![]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        // No event handling needed for now
        false
    }
}
