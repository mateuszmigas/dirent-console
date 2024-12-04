use std::io;

use crate::components::{App, Component, Node, RenderingContext};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    widgets::{Paragraph, WidgetRef},
    DefaultTerminal,
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
                let mut ctx = RenderingContext {
                    frame,
                    status_text: String::new(),
                };
                self.render_tree(&mut ctx, &mut nodes);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_tree(&self, ctx: &mut RenderingContext, nodes: &mut Vec<Node>) {
        let area = ctx.frame.area();

        nodes.clear();
        nodes.extend(self.root.render(ctx, area));

        for node in nodes.iter() {
            self.render_node(ctx, node);
        }
    }

    fn render_node(&self, ctx: &mut RenderingContext, node: &Node) {
        match node {
            Node::ComponentNode { component, area } => {
                let child_nodes = component.render(ctx, *area);
                for child in child_nodes.iter() {
                    self.render_node(ctx, child);
                }
            }
            Node::WidgetNode { widget, area } => {
                widget.render_ref(*area, ctx.frame.buffer_mut());
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
