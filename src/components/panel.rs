use crate::components::{input, Component, ComponentType, Input};
use crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::{
    component::{Node, RenderProps},
    RenderingContext,
};

#[derive(Debug)]
pub struct PanelProps {
    pub title: String,
    pub content: String,
}

impl RenderProps for PanelProps {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Panel {}

impl Panel {
    pub fn new() -> Self {
        Panel {}
    }
}

impl Component for Panel {
    fn render(&self, ctx: &mut RenderingContext, area: Rect, props: &dyn RenderProps) -> Vec<Node> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("hello".to_owned());

        let rect = Rect::new(20, 20, 100, 20);
        // ctx.frame.render_widget(Clear, rect);

        let paragraph = Paragraph::new("hehe".to_owned()).block(block);

        vec![
            Node::WidgetNode {
                widget: Box::new(Clear),
                area: rect,
            },
            Node::WidgetNode {
                widget: Box::new(paragraph),
                area: area,
            },
            Node::ComponentNode {
                component_type: ComponentType::Input(input::InputProps {
                    placeholder: "Input".to_string(),
                }),
                area: rect,
            },
        ]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        // No event handling needed for now
        false
    }
}
