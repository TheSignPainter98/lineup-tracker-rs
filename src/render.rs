use crate::model::{Ability, Map, Nameable, ProgressStore, Target, Usage, Zone};
use crate::selection::{Selection, Selector};
use tui::{
    layout::Constraint,
    style::{Color as Colour, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub trait Renderable<T> {
    fn render(&self) -> T;
}

impl<'a, T> Renderable<Cell<'a>> for T
where
    T: Nameable,
{
    fn render(&self) -> Cell<'a> {
        Cell::from(self.name().clone())
    }
}

impl<'a> Renderable<Cell<'a>> for Target {
    fn render(&self) -> Cell<'a> {
        let style = Style::default();
        let txt: String;

        // Get text and styling
        if self.target == 0 {
            style.fg(Colour::Blue);
            txt = "-".into();
        } else {
            txt = format!("{}/{}", self.progress, self.target);
            if self.target <= self.progress {
                style.fg(Colour::Green);
            } else if self.target >> 2 <= self.progress {
                style.fg(Colour::Yellow);
            } else {
                style.fg(Colour::Red);
            }
        };

        Cell::from(txt).style(style)
    }
}

impl<'a> Renderable<(usize, Table<'a>)> for ProgressStore {
    fn render(&self) -> (usize, Table<'a>) {
        let mut primary_hdr: Vec<Cell> = vec!["", ""].iter().map(|s| Cell::from(*s)).collect();
        let mut secondary_hdr: Vec<Cell> = vec!["", ""].iter().map(|s| Cell::from(*s)).collect();

        for m in &self.maps {
            let mut pushed_this_map_name = false;
            for z in &m.zones {
                primary_hdr.push(Cell::from(if pushed_this_map_name {
                    "".to_string()
                } else {
                    pushed_this_map_name = true;
                    m.name.clone()
                }));
                secondary_hdr.push(Cell::from(z.name.clone()));
            }
        }

        let err_style = Style::default().fg(Colour::Red);

        let ncols = primary_hdr.len();
        let mut rows = vec![Row::new(primary_hdr), Row::new(secondary_hdr)];
        for a in &self.abilities {
            let mut pushed_this_ability_name = false;
            for u in &a.usages {
                let mut row = Vec::new();
                row.push(Cell::from(if pushed_this_ability_name {
                    "".to_string()
                } else {
                    pushed_this_ability_name = true;
                    a.name.clone()
                }));
                row.push(Cell::from(u.name.clone()));
                for m in &self.maps {
                    for z in &m.zones {
                        let sel = Selection {
                            map: Some(Selector::Name(m.name.clone())),
                            zone: Some(Selector::Name(z.name.clone())),
                            ability: Some(Selector::Name(a.name.clone())),
                            usage: Some(Selector::Name(u.name.clone())),
                        };
                        row.push(match self.get_target(&sel) {
                            Some(t) => t.render(),
                            None => Cell::from("??".to_string()).style(err_style),
                        });
                    }
                }
                rows.push(Row::new(row));
            }
        }
        (
            ncols,
            Table::new(rows).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.name.clone()),
            ),
        )
    }
}
