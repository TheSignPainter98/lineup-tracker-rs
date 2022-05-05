use crate::model::{Ability, Map, Target, Usage, Zone};
use tui::{
    style::{Color as Colour, Style},
    widgets::Cell,
};

trait Renderable<T> {
    fn render(&self) -> T;
}

impl<'a> Renderable<Cell<'a>> for Map {
    fn render(&self) -> Cell<'a> {
        Cell::from(self.name.clone())
    }
}

impl<'a> Renderable<Cell<'a>> for Zone {
    fn render(&self) -> Cell<'a> {
        Cell::from(self.name.clone())
    }
}

impl<'a> Renderable<Cell<'a>> for Ability {
    fn render(&self) -> Cell<'a> {
        Cell::from(self.name.clone())
    }
}

impl<'a> Renderable<Cell<'a>> for Usage {
    fn render(&self) -> Cell<'a> {
        Cell::from(self.name.clone())
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
