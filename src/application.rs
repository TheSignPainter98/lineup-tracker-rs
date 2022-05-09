use crate::model::{Ability, Map, ProgressStore, Usage, Zone};
use crate::selection::{Selection, Selector};
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

enum InputSubject {
    MapName,
    ZoneName,
    AbilityName,
    UsageName,
}

enum InputOp {
    New,
    Remove,
}

type InputType = (InputOp, InputSubject);

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
    selection: Selection,
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
            selection: Selection::new(),
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
                            self.input_state = InputState::Edit(
                                (InputOp::Remove, InputSubject::MapName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('Z') => {
                            self.input_state = InputState::Edit(
                                (InputOp::Remove, InputSubject::ZoneName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('A') => {
                            self.input_state = InputState::Edit(
                                (InputOp::New, InputSubject::AbilityName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('U') => {
                            self.input_state = InputState::Edit(
                                (InputOp::Remove, InputSubject::UsageName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('m') => {
                            self.input_state = InputState::Edit(
                                (InputOp::New, InputSubject::MapName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('z') => {
                            self.input_state = InputState::Edit(
                                (InputOp::New, InputSubject::ZoneName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('a') => {
                            self.input_state = InputState::Edit(
                                (InputOp::New, InputSubject::AbilityName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Char('u') => {
                            self.input_state = InputState::Edit(
                                (InputOp::New, InputSubject::UsageName),
                                "".to_string(),
                                )
                        }
                        KeyCode::Down => self.next(),
                        KeyCode::Up => self.prev(),
                        _ => {}
                    },
                    InputState::Edit(ref op, ref mut buf) => match key.code {
                        KeyCode::Char(c) => buf.push(c),
                        KeyCode::Backspace => {
                            buf.pop();
                        }
                        KeyCode::Enter => {
                            match op {
                                (InputOp::New, InputSubject::MapName) => {
                                    self.progress.add_map(Map::new(buf.clone()));
                                    self.selection.map = Some(Selector::new(buf.clone()));
                                    self.selection.zone = None;
                                }
                                (InputOp::New, InputSubject::ZoneName) => {
                                    if let Some(mi) = &self.selection.map {
                                        let zone_selector = Selector::new(buf.clone());
                                        let zone = Zone::new(buf.clone());
                                        self.progress.add_zone(&mi, &zone_selector);
                                        self.selection.zone = Some(zone_selector);

                                        if let Some(map) = mi.get_selected_mut::<Map>(&mut self.progress.maps) {
                                            map.add_zone(zone);
                                        }
                                    } else {
                                        println!("No map!");
                                    }
                                }
                                (InputOp::New, InputSubject::AbilityName) => {
                                    self.progress.add_ability(Ability::new(buf.clone()));
                                    self.selection.ability = Some(Selector::new(buf.clone()));
                                    self.selection.usage = None;
                                }
                                (InputOp::New, InputSubject::UsageName) => {
                                    if let Some(asel) = &self.selection.ability {
                                        let usage_selector = Selector::new(buf.clone());
                                        let usage = Usage::new(buf.clone());
                                        self.progress.add_usage(&asel, &usage_selector);
                                        self.selection.usage = Some(usage_selector);

                                        if let Some(ability) = asel.get_selected_mut(&mut self.progress.abilities) {
                                            ability.add_usage(usage);
                                        }
                                    }
                                }
                                (InputOp::Remove, _) => {} // TODO: implement removal
                            };
                            println!(
                                "Current input state is {:?} {:?} {:?} {:?}",
                                self.selection.map,
                                self.selection.zone,
                                self.selection.ability,
                                self.selection.usage
                                );
                            self.input_state = InputState::Normal;
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
        if let InputState::Edit(_, _) = self.input_state {
            rect_constraints = [Constraint::Percentage(95), Constraint::Min(3)].as_ref();
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

        if let InputState::Edit(t, s) = &self.input_state {
            let mut box_name = match t {
                (InputOp::New, _) => "New ",
                (InputOp::Remove, _) => "Remove ",
            }
            .to_string();
            box_name.push_str(match t {
                (_, InputSubject::MapName) => "Map",
                (_, InputSubject::ZoneName) => "Zone",
                (_, InputSubject::AbilityName) => "Ability",
                (_, InputSubject::UsageName) => "Usage",
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
