use crate::components::{panel::PanelProps, Component, ComponentType};
use crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::{
    component::{Node, RenderProps},
    RenderingContext,
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for App {
    fn render(&self, context: &RenderingContext, _: &dyn RenderProps) -> Vec<Node> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(context.area);

        vec![
            Node::ComponentNode {
                component_type: ComponentType::Panel(PanelProps {
                    title: "Left Panel".to_string(),
                }),
                area: chunks[0],
            },
            Node::ComponentNode {
                component_type: ComponentType::Panel(PanelProps {
                    title: "Right Panel".to_string(),
                }),
                area: chunks[1],
            },
        ]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        false
    }
}
