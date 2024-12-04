use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event);
}
