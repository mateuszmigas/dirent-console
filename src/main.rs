// ANCHOR: all
use std::io;
mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result: Result<(), io::Error> = crate::app::App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
