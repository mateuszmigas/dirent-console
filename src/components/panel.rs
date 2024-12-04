use crate::components::{input, Component, ComponentType};
use crossterm::event::Event;
use ratatui::prelude::*;

use super::{
    component::{Node, RenderProps},
    RenderingContext,
};

#[derive(Debug)]
pub struct PanelProps {
    pub title: String,
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
    fn render(&self, context: &RenderingContext, props: &dyn RenderProps) -> Vec<Node> {
        let panel_props = props.as_any().downcast_ref::<PanelProps>().unwrap();
        vec![Node::ComponentNode {
            component_type: ComponentType::Input(input::InputProps {
                initial_value: panel_props.title.clone() + "_input",
            }),
            area: context.area,
        }]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        // No event handling needed for now
        false
    }
}
