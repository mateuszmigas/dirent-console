use crate::components::{input, Component, Input};
use crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::{component::Node, RenderingContext};

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
    fn render(&self, ctx: &mut RenderingContext, area: Rect) -> Vec<Node> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.clone().unwrap_or_default());

        let rect = Rect::new(20, 20, 100, 20);
        // ctx.frame.render_widget(Clear, rect);

        let paragraph = Paragraph::new(self.content.clone()).block(block);

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
                component: Box::new(Input::new()),
                area: rect,
            },
        ]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        // No event handling needed for now
        false
    }
}
