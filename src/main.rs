mod application;
mod model;
mod render;
mod selection;

use crate::application::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs::File;
use std::io::{stdout, Result};
use std::io::{BufReader, BufWriter};
use tui::{backend::CrosstermBackend, Terminal};

const SAVE_LOC: &str = "./.progress-rs.json";

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = match File::open(SAVE_LOC) {
        Ok(f) => {
            let r = BufReader::new(f);
            App::load(r)?
        }
        Err(_) => App::new(),
    };
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let f = File::create(SAVE_LOC)?;
    app.save(BufWriter::new(f))?;

    res
}
