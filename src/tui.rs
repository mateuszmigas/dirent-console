use std::io;

use crate::components::{
    App, AppProps, Component, ComponentType, Input, Node, Panel, RenderingContext,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{layout::Rect, DefaultTerminal, Frame};

pub struct Tui {
    exit: bool,
}

impl Default for Tui {
    fn default() -> Self {
        Self { exit: false }
    }
}

impl Tui {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.render_root(frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_root(&self, frame: &mut Frame) {
        let area = frame.area();

        self.render_node(
            frame,
            &Node::ComponentNode {
                component_type: ComponentType::App(AppProps {}),
                area,
            },
        );
    }

    fn render_node(&self, frame: &mut Frame, node: &Node) {
        match node {
            Node::ComponentNode {
                component_type,
                area,
            } => {
                let child_nodes = render_component(
                    component_type,
                    &RenderingContext {
                        area: *area,
                        window_area: frame.area(),
                    },
                );
                for child in child_nodes.iter() {
                    self.render_node(frame, child);
                }
            }
            Node::WidgetNode { widget, area } => {
                widget.render_ref(*area, frame.buffer_mut());
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                if key_event.code == KeyCode::Char('q') {
                    self.exit = true;
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

fn render_component(component_type: &ComponentType, context: &RenderingContext) -> Vec<Node> {
    match component_type {
        ComponentType::Input(props) => Input::new().render(context, props),
        ComponentType::Panel(props) => Panel::new().render(context, props),
        ComponentType::App(props) => App::new().render(context, props),
    }
}
