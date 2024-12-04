use super::component::{Component, Node, RenderProps};
use crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug)]
pub struct InputProps {
    pub initial_value: String,
}

impl RenderProps for InputProps {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Input {}
    }
}

impl Component for Input {
    fn render(&self, props: &dyn RenderProps, area: Rect) -> Vec<Node> {
        let input_props = props.as_any().downcast_ref::<InputProps>().unwrap();
        let block = Block::default().borders(Borders::ALL);
        let paragraph = Paragraph::new(input_props.initial_value.clone()).block(block);
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
