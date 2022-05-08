use crate::model::{Ability, Map, ProgressStore};
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color as Colour, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

enum InputType {
    MapName,
    ZoneName,
    AbilityName,
    UsageName,
}

enum InputState {
    Normal,
    Edit(InputType, String),
}

pub struct App<'a> {
    state: TableState,
    pub progress: ProgressStore,
    items: Vec<Vec<&'a str>>,
    ncols: usize,
    input_state: InputState,
}

impl<'a> App<'a> {
    pub fn new(data: Vec<Vec<&'a str>>) -> App<'a> {
        let app = App {
            state: TableState::default(),
            ncols: match data.get(0) {
                Some(row) => row.len(),
                None => 0,
            },
            items: data,
            progress: ProgressStore::new("Progress".into()),
            input_state: InputState::Normal,
        };
        app
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            if let Event::Key(key) = event::read()? {
                match self.input_state {
                    InputState::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('+') => {}
                        KeyCode::Char('m') => {
                            self.input_state = InputState::Edit(InputType::MapName, "".to_string())
                        }
                        KeyCode::Char('z') => {
                            self.input_state = InputState::Edit(InputType::ZoneName, "".to_string())
                        }
                        KeyCode::Char('a') => {
                            self.input_state =
                                InputState::Edit(InputType::AbilityName, "".to_string())
                        }
                        KeyCode::Char('u') => {
                            self.input_state = InputState::Edit(InputType::UsageName, "".to_string())
                        }
                        KeyCode::Down => self.next(),
                        KeyCode::Up => self.prev(),
                        _ => {}
                    },
                    InputState::Edit(ref itype, ref mut buf) => match key.code {
                        KeyCode::Char(c) => buf.push(c),
                        KeyCode::Backspace => {
                            buf.pop();
                        }
                        KeyCode::Enter => match itype {
                            InputType::MapName => self.progress.new_map(Map::new(buf.clone())),
                            InputType::ZoneName => {}
                            InputType::AbilityName => {
                                self.progress.new_ability(Ability::new(buf.clone()))
                            }
                            InputType::UsageName => {}
                        },
                        KeyCode::Esc => self.input_state = InputState::Normal,
                        _ => {}
                    },
                }
            }
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
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
        let rows = self.items.iter().map(|item| {
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

        let widths = [Constraint::Percentage(100 / self.ncols as u16)].repeat(self.ncols);
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .highlight_style(style_selected)
            .highlight_symbol(">> ")
            .widths(&widths);
        f.render_stateful_widget(t, rects[0], &mut self.state);
    }

    fn next(&mut self) {
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

    fn prev(&mut self) {
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
