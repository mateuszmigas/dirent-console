use super::{
    component::{Component, Node, RenderProps},
    RenderingContext,
};
use crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug)]
pub struct InputProps {
    pub placeholder: String,
}

impl RenderProps for InputProps {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Input {
    value: String,
    placeholder: String,
}

impl Input {
    pub fn new() -> Self {
        Input {
            value: "dupa input".to_string(),
            placeholder: "".to_string(),
        }
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl Component for Input {
    fn render(&self, ctx: &mut RenderingContext, area: Rect, props: &dyn RenderProps) -> Vec<Node> {
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(self.value.clone()).block(block);
        vec![Node::WidgetNode {
            widget: Box::new(paragraph),
            area: area,
        }]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        // No event handling needed for now
        false
    }
}
