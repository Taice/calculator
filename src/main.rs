use std::io::{self};

mod app;
mod calculator;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app::run(terminal);
    ratatui::restore();
    app_result
    // test
}
