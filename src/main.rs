use app::App;
use std::io::{self};

mod app;
mod calculator;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::run(&mut App::default(), terminal);
    ratatui::restore();
    app_result
}
