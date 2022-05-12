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

    pub fn absolute(&self, maps: &Vec<Map>, abilities: &Vec<Ability>) -> Self {
        let mut nmap = None;
        let mut nzone = None;
        let mut nability = None;
        let mut nusage = None;
        if let Some(map) = &self.map {
            if let (Some(zone), Some(m)) = (&self.zone, map.get_selected(maps)) {
                nzone = zone.to_index(&m.zones);
            }
            nmap = map.to_index(maps);
        }

        if let Some(ability) = &self.ability {
            if let (Some(usage), Some(a)) = (&self.usage, ability.get_selected(abilities)) {
                nusage = usage.to_index(&a.usages);
            }
            nability = ability.to_index(abilities);
        }
        Self {
            map: nmap,
            zone: nzone,
            ability: nability,
            usage: nusage,
        }
    }

    pub fn relative(&self, maps: &Vec<Map>, abilities: &Vec<Ability>) -> Self {
        let mut nmap = None;
        let mut nzone = None;
        let mut nability = None;
        let mut nusage = None;
        if let Some(map) = &self.map {
            if let (Some(zone), Some(m)) = (&self.zone, map.get_selected(maps)) {
                nzone = zone.to_name(&m.zones);
            }
            nmap = map.to_name(maps);
        }

        if let Some(ability) = &self.ability {
            if let (Some(usage), Some(a)) = (&self.usage, ability.get_selected(abilities)) {
                nusage = usage.to_name(&a.usages);
            }
            nability = ability.to_name(abilities);
        }
        Self {
            map: nmap,
            zone: nzone,
            ability: nability,
            usage: nusage,
        }
    }

    pub fn next_zone(&mut self, maps: &Vec<Map>) {
        if let Some(map) = &self.map {
            self.map = map.to_index(maps);
            if let Some(Some(m)) = self.map.as_ref().map(|m| m.get_selected(maps)) {
                if let Some(zone) = &self.zone {
                    self.zone = zone.to_index(&m.zones);
                }
            }
        }

        if let (Some(msel), Some(Selector::Index(zidx))) = (&self.map, &self.zone) {
            if let Some(m) = msel.get_selected(maps) {
                if *zidx < m.zones.len() - 1 {
                    self.zone = Some(Selector::Index(zidx + 1));
                } else if let Selector::Index(midx) = msel {
                    self.map = Some(Selector::Index((midx + 1) % maps.len()));
                    self.zone = Some(Selector::Index(0));
                }
            }
        }
    }

    pub fn prev_zone(&mut self, maps: &Vec<Map>) {
        if let Some(map) = &self.map {
            self.map = map.to_index(maps);
            if let Some(Some(m)) = self.map.as_ref().map(|m| m.get_selected(maps)) {
                if let Some(zone) = &self.zone {
                    self.zone = zone.to_index(&m.zones);
                }
            }
        }

        if let (Some(msel), Some(Selector::Index(zidx))) = (&self.map, &self.zone) {
            if *zidx == 0 {
                if let Selector::Index(midx) = msel {
                    let new_msel = if *midx == 0 {
                        Selector::Index(maps.len() - 1)
                    } else {
                        Selector::Index(midx - 1)
                    };
                    self.zone = Some(Selector::Index(new_msel.get_selected(maps).unwrap().zones.len() - 1));
                    self.map = Some(new_msel);
                }
            } else {
                self.zone = Some(Selector::Index(zidx - 1));
            }
        }
    }

    pub fn next_usage(&mut self, abilities: &Vec<Ability>) {
        if let Some(ability) = &self.ability {
            self.ability = ability.to_index(abilities);
            if let Some(Some(a)) = self.ability.as_ref().map(|m| m.get_selected(abilities)) {
                if let Some(usage) = &self.usage {
                    self.usage = usage.to_index(&a.usages);
                }
            }
        }

        if let (Some(asel), Some(Selector::Index(uidx))) = (&self.ability, &self.usage) {
            if let Some(a) = asel.get_selected(abilities) {
                if *uidx < a.usages.len() - 1 {
                    self.usage = Some(Selector::Index(uidx + 1));
                } else if let Selector::Index(aidx) = asel {
                    self.ability = Some(Selector::Index((aidx + 1) % abilities.len()));
                    self.usage = Some(Selector::Index(0));
                }
            }
        }
    }

    pub fn prev_usage(&mut self, abilities: &Vec<Ability>) {
        if let Some(ability) = &self.ability {
            self.ability = ability.to_index(abilities);
            if let Some(Some(a)) = self.ability.as_ref().map(|m| m.get_selected(abilities)) {
                if let Some(usage) = &self.usage {
                    self.usage = usage.to_index(&a.usages);
                }
            }
        }

        if let (Some(asel), Some(Selector::Index(uidx))) = (&self.ability, &self.usage) {
            if *uidx == 0 {
                if let Selector::Index(aidx) = asel {
                    let new_asel = if *aidx == 0 {
                        Selector::Index(abilities.len() - 1)
                    } else {
                        Selector::Index(aidx - 1)
                    };
                    self.usage = Some(Selector::Index(new_asel.get_selected(abilities).unwrap().usages.len() - 1));
                    self.ability = Some(new_asel);
                }
            } else {
                self.usage = Some(Selector::Index(uidx - 1));
            }
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            map: Some(Selector::default()),
            zone: Some(Selector::default()),
            ability: Some(Selector::default()),
            usage: Some(Selector::default()),
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
        self.get_selected_idx(vs)
            .map(|i| Selector::Name(vs[i].name().clone()))
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
