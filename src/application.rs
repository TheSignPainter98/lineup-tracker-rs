use crate::model::{Ability, Map, ProgressStore};
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color as Colour, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputType {
    MapName,
    ZoneName,
    AbilityName,
    UsageName,
}

enum InputOp {
    New,
    Remove,
}

enum InputState {
    Normal,
    Edit(InputType, InputOp, String),
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
                        KeyCode::Char('M') => {
                            self.input_state =
                                InputState::Edit(InputType::MapName, InputOp::Remove, "".to_string())
                        }
                        KeyCode::Char('Z') => {
                            self.input_state =
                                InputState::Edit(InputType::ZoneName, InputOp::Remove, "".to_string())
                        }
                        KeyCode::Char('A') => {
                            self.input_state = InputState::Edit(
                                InputType::AbilityName,
                                InputOp::New,
                                "".to_string(),
                            )
                        }
                        KeyCode::Char('U') => {
                            self.input_state =
                                InputState::Edit(InputType::UsageName, InputOp::Remove, "".to_string())
                        }
                        KeyCode::Char('m') => {
                            self.input_state =
                                InputState::Edit(InputType::MapName, InputOp::New, "".to_string())
                        }
                        KeyCode::Char('z') => {
                            self.input_state =
                                InputState::Edit(InputType::ZoneName, InputOp::New, "".to_string())
                        }
                        KeyCode::Char('a') => {
                            self.input_state = InputState::Edit(
                                InputType::AbilityName,
                                InputOp::New,
                                "".to_string(),
                            )
                        }
                        KeyCode::Char('u') => {
                            self.input_state = InputState::Edit(
                                InputType::UsageName,
                                InputOp::New,
                                "".to_string(),
                            )
                        }
                        KeyCode::Down => self.next(),
                        KeyCode::Up => self.prev(),
                        _ => {}
                    },
                    InputState::Edit(ref itype, ref op, ref mut buf) => match key.code {
                        KeyCode::Char(c) => buf.push(c),
                        KeyCode::Backspace => {
                            buf.pop();
                        }
                        KeyCode::Enter => {
                            match (op, itype) {
                                (InputOp::New, InputType::MapName) => self.progress.new_map(Map::new(buf.clone())),
                                (InputOp::New, InputType::ZoneName) => {}, // TODO: implement zone addition
                                (InputOp::New, InputType::AbilityName) => self.progress.new_ability(Ability::new(buf.clone())),
                                (InputOp::New, InputType::UsageName) => {}, // TODO: implement zone addition
                                (InputOp::Remove, _) => {}, // TODO: implement removal
                            };
                            self.input_state = InputState::Normal
                        }
                        KeyCode::Esc => self.input_state = InputState::Normal,
                        _ => {}
                    },
                }
            }
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let rect_constraints;
        if let InputState::Edit(_, _, _) = self.input_state {
            rect_constraints = [Constraint::Percentage(95), Constraint::Min(1)].as_ref();
        } else {
            rect_constraints = [Constraint::Percentage(100)].as_ref();
        }
        let rects = Layout::default()
            .margin(4)
            .constraints(rect_constraints)
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

        if let InputState::Edit(t, o, s) = &self.input_state {
            let mut box_name = match o {
                InputOp::New => "New ",
                InputOp::Remove => "Remove ",
            }.to_string();
            box_name.push_str(match t {
                InputType::MapName => "Map",
                InputType::ZoneName => "Zone",
                InputType::AbilityName => "Ability",
                InputType::UsageName => "Usage",
            });
            let input_box = Paragraph::new(s.as_ref())
                .block(Block::default().borders(Borders::ALL).title(box_name));
            f.render_widget(input_box, rects[1]);
            f.set_cursor(rects[1].x + s.width() as u16 + 1, rects[1].y + 1)
        }
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
