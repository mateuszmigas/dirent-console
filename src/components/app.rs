use crate::components::{Component, ComponentType, Panel, RenderingContext};
use crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::component::Node;

pub struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl Component for App {
    fn render(&self, ctx: &mut RenderingContext, area: Rect) -> Vec<Node> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        vec![
            Node::ComponentNode {
                component_type: ComponentType::Panel,
                area: chunks[0],
            },
            Node::ComponentNode {
                component_type: ComponentType::Panel,
                area: chunks[1],
            },
            // Node::ComponentNode {
            //     component: Box::new(Panel::new(chunks[0]).with_content("Left panel content")),
            //     area: chunks[0],
            // },
            // Node::ComponentNode {
            //     component: Box::new(
            //         Panel::new(chunks[1])
            //             .with_title("Right Panel")
            //             .with_content("Right panel content"),
            //     ),
            //     area: chunks[1],
            // },
        ]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        false
    }
}
