use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

pub struct RenderingContext<'a, 'b> {
    pub frame: &'a mut Frame<'b>,
    pub status_text: String,
}

/// A Node represents a component in the UI tree
pub struct Node {
    pub component: Box<dyn Component>,
    pub area: Rect,
}

pub trait Component {
    /// Render this component and return its child components
    fn render(&self, context: &mut RenderingContext, area: Rect) -> Vec<Node>;

    /// Handle an event, return true if handled
    fn handle_event(&mut self, event: Event) -> bool {
        false
    }
}
