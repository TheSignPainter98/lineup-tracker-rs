use crate::model::{Ability, Map, Nameable};
use serde::{Deserialize as Deserialise, Serialize as Serialise};

#[derive(Eq, PartialEq, Debug, Serialise, Deserialise)]
pub struct Selection {
    pub map: Option<Selector>,
    pub zone: Option<Selector>,
    pub ability: Option<Selector>,
    pub usage: Option<Selector>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            map: None,
            zone: None,
            ability: None,
            usage: None,
        }
    }

    pub fn make_absolute(&mut self, maps: &Vec<Map>, abilities: &Vec<Ability>) {
        if let Some(map) = &self.map {
            if let (Some(zone), Some(m)) = (&self.zone, map.get_selected(maps)) {
                self.zone = zone.to_index(&m.zones);
            }
            self.map = map.to_index(maps);
        }

        if let Some(ability) = &self.ability {
            if let (Some(usage), Some(a)) = (&self.usage, ability.get_selected(abilities)) {
                self.usage = usage.to_index(&a.usages);
            }
            self.ability = ability.to_index(abilities);
        }
    }

    pub fn make_relative(&mut self, maps: &Vec<Map>, abilities: &Vec<Ability>) {
        if let Some(map) = &self.map {
            if let (Some(zone), Some(m)) = (&self.zone, map.get_selected(maps)) {
                self.zone = zone.to_name(&m.zones);
            }
            self.map = map.to_name(maps);
        }

        if let Some(ability) = &self.ability {
            if let (Some(usage), Some(a)) = (&self.usage, ability.get_selected(abilities)) {
                self.usage = usage.to_name(&a.usages);
            }
            self.ability = ability.to_name(abilities);
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialise, Deserialise)]
pub enum Selector {
    Index(usize),
    Name(String),
}

impl Selector {
    pub fn get_selected<'a, S>(&self, vs: &'a Vec<S>) -> Option<&'a S>
    where
        S: Nameable,
        // T: SliceIndex<usize, Output=S> + IntoIterator<Item = S>,
    {
        self.get_selected_idx(vs).map(|i| &vs[i])
    }

    pub fn get_selected_mut<'a, S>(&self, vs: &'a mut Vec<S>) -> Option<&'a mut S>
    where
        S: Nameable,
        // T: SliceIndex<usize, Output=S> + IntoIterator<Item = S>,
    {
        self.get_selected_idx(vs).map(|i| &mut vs[i])
    }

    fn get_selected_idx<S>(&self, vs: &Vec<S>) -> Option<usize>
    where
        S: Nameable,
        // T: SliceIndex<usize, Output=S> + IntoIterator<Item = S>,
    {
        match self {
            Selector::Name(name) => {
                for (i, u) in vs.iter().enumerate() {
                    if u.name() == name {
                        return Some(i);
                    }
                }
                None
            }
            Selector::Index(idx) => {
                if *idx < vs.len() {
                    Some(*idx)
                } else {
                    None
                }
            }
        }
    }

    pub fn to_index<S>(&self, vs: &Vec<S>) -> Option<Selector>
    where
        S: Nameable,
        // T: SliceIndex<usize, Output=S> + IntoIterator<Item = S>,
    {
        self.get_selected_idx(vs).map(|i| Selector::Index(i))
    }

    pub fn to_name<S>(&self, vs: &Vec<S>) -> Option<Selector>
    where
        S: Nameable,
        // T: SliceIndex<usize, Output=S> + IntoIterator<Item = S>,
    {
        self.get_selected_idx(vs).map(|i| Selector::Name(vs[i].name().clone()))
    }
}

impl Default for Selector {
    fn default() -> Self {
        Selector::Index(0)
    }
}

impl From<String> for Selector {
    fn from(s: String) -> Self {
        match s.parse::<usize>() {
            Ok(n) => Self::Index(n),
            Err(_) => Self::Name(s),
        }
    }
}
