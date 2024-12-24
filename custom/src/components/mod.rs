mod app;
mod component;
mod input;
mod panel;

pub use app::App;
pub use component::{Component, Node, RenderProps, RenderingContext};
pub use input::{Input, InputProps};
pub use panel::{Panel, PanelProps};

pub enum ComponentType {
    Panel(panel::PanelProps),
    Input(input::InputProps),
    App,
}
