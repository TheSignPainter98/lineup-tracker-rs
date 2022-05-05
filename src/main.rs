mod model;
mod render;
mod selection;

use crate::model::ProgressStore;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::{
    error::Error,
    io::{self, stdout},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color as Colour, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

mod model;
mod render;

struct App<'a> {
    state: TableState,
    progress: ProgressStore,
    items: Vec<Vec<&'a str>>,
    ncols: usize,
}

impl<'a> App<'a> {
    fn new(data: Vec<Vec<&'a str>>) -> App<'a> {
        let app = App {
            state: TableState::default(),
            ncols: match data.get(0) {
                Some(row) => row.len(),
                None => 0,
            },
            items: data,
            progress: ProgressStore::new("Progress".into()),
        };
        app
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() - 1 <= i {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data = vec![vec!["Hello", "there", "world!"], vec!["How", "are", "you?"]];
    let app = App::new(data);
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.prev(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(4)
        .split(f.size());

    let style_selected = Style::default().add_modifier(Modifier::REVERSED);
    let style_normal = Style::default().fg(Colour::Blue);
    let style_header = Style::default()
        .fg(Colour::White)
        .add_modifier(Modifier::BOLD);
    let header_cells = ["Head1", "Head2", "Head3"]
        .iter()
        .map(|h| Cell::from(*h).style(style_header));
    let header = Row::new(header_cells)
        .style(style_normal)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells)
            .height(height as u16)
            .bottom_margin(1)
            .style(style_normal)
    });

    let widths = [Constraint::Percentage(100 / app.ncols as u16)].repeat(app.ncols);
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .highlight_style(style_selected)
        .highlight_symbol(">> ")
        .widths(&widths);
    f.render_stateful_widget(t, rects[0], &mut app.state);

    model::Ability::new("fdasf".to_string());
}
