use std::io;

use crate::components::{App, Component, RenderingContext};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

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
        while !self.exit {
            terminal.draw(|frame| {
                let mut ctx = RenderingContext {
                    frame,
                    status_text: String::new(),
                };
                self.render_tree(&mut ctx);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_tree(&self, ctx: &mut RenderingContext) {
        let area = ctx.frame.area();
        let mut nodes = self.root.render(ctx, area);
        
        // Process the render tree
        while let Some(node) = nodes.pop() {
            // Get children of this node
            let mut children = node.component.render(ctx, node.area);
            nodes.extend(children);
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
