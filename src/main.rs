use std::io;

mod components;
mod macros;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let tui_result: Result<(), io::Error> = crate::tui::Tui::default().run(&mut terminal);
    ratatui::restore();
    tui_result
}
