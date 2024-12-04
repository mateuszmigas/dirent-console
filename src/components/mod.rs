mod app;
mod component;
mod input;
mod panel;

pub use app::{App, AppProps};
pub use component::{Component, Node, RenderingContext};
pub use input::{Input, InputProps};
pub use panel::{Panel, PanelProps};

pub enum ComponentType {
    Panel(panel::PanelProps),
    Input(input::InputProps),
    App(app::AppProps),
}
