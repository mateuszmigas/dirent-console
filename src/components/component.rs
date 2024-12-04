use crossterm::event::Event;
use ratatui::{layout::Rect, widgets::WidgetRef};

use super::ComponentType;

pub struct RenderingContext {
    pub area: Rect,
    pub window_area: Rect,
}

pub enum Node {
    ComponentNode {
        component_type: ComponentType,
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
