use crate::components::{Component, Panel};
use crossterm::event::Event;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct App {}

impl Component for App {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let panel_left = Panel::new(area)
            .with_title("My Panel Left")
            .with_content("Hello, this is some text inside the panel left!");

        panel_left.render(frame, chunks[0]);

        let panel_right = Panel::new(area)
            .with_title("My Panel Right")
            .with_content("Hello, this is some text inside the panel right!");

        panel_right.render(frame, chunks[2]);
    }

    fn handle_event(&mut self, event: Event) {
        // No event handling needed for now
    }
}
