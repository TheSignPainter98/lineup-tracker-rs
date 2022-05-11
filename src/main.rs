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
use shellexpand::tilde;

const SAVE_LOC: &str = "~/.lineup-progress-rs.json";

fn main() -> Result<()> {
    enable_raw_mode()?;
    let save_loc: String = tilde(SAVE_LOC).into();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = match File::open(&save_loc) {
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

    let f = File::create(&save_loc)?;
    app.save(BufWriter::new(f))?;

    res
}
