use crate::model::{Nameable, ProgressStore, Target};
use crate::selection::{Selection, Selector};
use tui::{
    style::{Color as Colour, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub trait Renderable<T, Selector = bool> {
    fn render(&self, selected: Selector) -> T;
}

impl<'a, T> Renderable<Cell<'a>, ()> for T
where
    T: Nameable,
{
    fn render(&self, _: ()) -> Cell<'a> {
        Cell::from(self.name().clone())
    }
}

impl<'a> Renderable<Cell<'a>> for Target {
    fn render(&self, selected: bool) -> Cell<'a> {
        let mut style = Style::default();
        let txt: String;

        // Get text and styling
        if self.target == 0 {
            txt = "-".into();
            style = style.fg(Colour::Blue);
        } else {
            txt = format!("{}/{}", self.progress, self.target);
            style = style.fg(if self.target <= self.progress {
                Colour::Green
            } else if self.target >> 1 <= self.progress.abs() {
                Colour::Yellow
            } else {
                Colour::Red
            })
        };

        if selected {
            style = style
                .fg(Colour::Black)
                .bg(Colour::White)
                .add_modifier(Modifier::BOLD);
        }

        Cell::from(txt).style(style)
    }
}

impl<'a> Renderable<(usize, Table<'a>), &Selection> for ProgressStore {
    fn render(&self, selected: &Selection) -> (usize, Table<'a>) {
        let abs_selected = selected.absolute(&self.maps, &self.abilities);
        let mut primary_hdr: Vec<Cell> = vec!["", ""].iter().map(|s| Cell::from(*s)).collect();
        let mut secondary_hdr: Vec<Cell> = vec!["", ""].iter().map(|s| Cell::from(*s)).collect();

        for a in &self.abilities {
            let mut pushed_this_ability_name = false;
            for u in &a.usages {
                primary_hdr.push(Cell::from(if pushed_this_ability_name {
                    "".to_string()
                } else {
                    pushed_this_ability_name = true;
                    a.name.clone()
                }));
                secondary_hdr.push(Cell::from(u.name.clone()));
            }
        }

        let err_style = Style::default().fg(Colour::Red);

        let ncols = primary_hdr.len();
        let mut rows = vec![Row::new(primary_hdr), Row::new(secondary_hdr)];
        for (mi, m) in self.maps.iter().enumerate() {
            let mut pushed_this_map_name = false;
            for (zi, z) in m.zones.iter().enumerate() {
                let mut row = Vec::new();
                row.push(Cell::from(if pushed_this_map_name {
                    "".to_string()
                } else {
                    pushed_this_map_name = true;
                    m.name.clone()
                }));
                row.push(Cell::from(z.name.clone()));
                for (ai, a) in self.abilities.iter().enumerate() {
                    for (ui, _) in a.usages.iter().enumerate() {
                        let sel = Selection {
                            map: Some(Selector::Index(mi)),
                            zone: Some(Selector::Index(zi)),
                            ability: Some(Selector::Index(ai)),
                            usage: Some(Selector::Index(ui)),
                        };

                        row.push(match self.get_target(&sel) {
                            Some(t) => t.render(sel == abs_selected),
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
