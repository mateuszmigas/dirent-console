use crate::components::{input, Component, ComponentType};
use crossterm::event::Event;
use ratatui::{prelude::*, widgets::Block};

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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ])
            .margin(1)
            .split(context.area);

        let panel_props = props.as_any().downcast_ref::<PanelProps>().unwrap();
        vec![
            Node::WidgetNode {
                widget: Box::new(
                    Block::bordered()
                        .style(Style::new().blue().on_white().bold().italic())
                        .title(chunks[0].to_string()),
                ),
                area: chunks[0],
            },
            Node::ComponentNode {
                component_type: ComponentType::Input(input::InputProps {
                    initial_value: panel_props.title.clone() + "_input",
                }),
                area: chunks[1],
            },
            Node::WidgetNode {
                widget: Box::new(
                    Block::bordered()
                        .style(Style::new().blue().on_white().bold().italic())
                        .title(chunks[2].to_string()),
                ),
                area: chunks[2],
            },
        ]
    }

    fn handle_event(&mut self, event: Event) -> bool {
        // No event handling needed for now
        false
    }
}
