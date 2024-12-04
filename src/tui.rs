use std::io;

use crate::components::{App, Component, Node, RenderingContext};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    widgets::{Paragraph, WidgetRef},
    DefaultTerminal, Frame,
};

pub struct Tui {
    root: App,
    exit: bool,
}

impl Default for Tui {
    fn default() -> Self {
        Self {
            root: App::default(),
            exit: false,
        }
    }
}

impl Tui {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut nodes = vec![];
        while !self.exit {
            terminal.draw(|frame| {
                self.render_tree(frame, &mut nodes);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_tree(&self, frame: &mut Frame, nodes: &mut Vec<Node>) {
        let area = frame.area();

        nodes.clear();
        nodes.extend(self.root.render(
            &mut RenderingContext {
                area,
                window_area: area,
            },
            area,
        ));

        for node in nodes.iter() {
            self.render_node(
                frame,
                &mut RenderingContext {
                    area,
                    window_area: area,
                },
                node,
            );
        }
    }

    fn render_node(
        &self,
        frame: &mut Frame,
        rendering_context: &mut RenderingContext,
        node: &Node,
    ) {
        match node {
            Node::ComponentNode { component, area } => {
                let child_nodes = component.render(rendering_context, *area);
                for child in child_nodes.iter() {
                    self.render_node(
                        frame,
                        &mut RenderingContext {
                            area: *area,
                            window_area: rendering_context.window_area,
                        },
                        child,
                    );
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

                self.root.handle_event(Event::Key(key_event));
            }
        }
        Ok(())
    }
}
