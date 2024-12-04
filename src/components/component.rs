use crossterm::event::Event;
use ratatui::{
    layout::Rect,
    widgets::{Widget, WidgetRef},
    Frame,
};

pub struct RenderingContext<'a, 'b> {
    pub frame: &'a mut Frame<'b>,
    pub status_text: String,
}

pub enum Node {
    ComponentNode {
        component: Box<dyn Component>,
        area: Rect,
    },
    WidgetNode {
        widget: Box<dyn WidgetRef>,
        area: Rect,
    },
}

pub trait Component {
    fn render(&self, context: &mut RenderingContext, area: Rect) -> Vec<Node>;

    fn handle_event(&mut self, event: Event) -> bool {
        false
    }
}
