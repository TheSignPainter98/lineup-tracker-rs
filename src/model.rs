use crate::selection::{Selection, Selector};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub trait Nameable {
    fn name<'a>(&'a self) -> &'a String;
}

#[derive(Eq)]
pub struct Map {
    pub name: String,
    pub zones: Vec<Zone>,
}

impl Map {
    pub fn new(name: String) -> Self {
        Map {
            name: name,
            zones: Vec::new(),
        }
    }

    pub fn add_zone(&mut self, z: Zone) {
        self.zones.push(z)
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.name.hash(h);
    }
}

impl Nameable for Map {
    fn name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Zone {
    pub name: String,
}

impl Zone {
    pub fn new(name: String) -> Self {
        Zone { name: name }
    }
}

impl Nameable for Zone {
    fn name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

#[derive(Eq)]
pub struct Ability {
    pub name: String,
    pub usages: Vec<Usage>,
}

impl Ability {
    pub fn new(name: String) -> Self {
        Ability {
            name: name,
            usages: Vec::new(),
        }
    }

    pub fn add_usage(&mut self, u: Usage) {
        self.usages.push(u);
    }
}

impl PartialEq for Ability {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Nameable for Ability {
    fn name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

impl Hash for Ability {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.name.hash(h);
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Usage {
    pub name: String,
}

impl Usage {
    pub fn new(name: String) -> Usage {
        Usage { name: name }
    }
}

impl Nameable for Usage {
    fn name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

pub struct Target {
    pub progress: i32,
    pub target: i32,
}

impl Target {
    pub fn new(amt: Option<i32>, target: Option<i32>) -> Target {
        Target {
            progress: amt.unwrap_or(0),
            target: target.unwrap_or(2),
        }
    }

    pub fn zero_progress(&mut self) {
        self.progress = 0;
    }

    pub fn zero_target(&mut self) {
        self.target = 0;
    }

    pub fn match_progress_tp_target(&mut self) {
        self.progress = self.target;
    }

    pub fn match_target_to_progress(&mut self) {
        self.target = self.progress;
    }

    pub fn change_progress(&mut self, delta: i32) {
        self.progress += delta;
    }

    pub fn change_target(&mut self, delta: i32) {
        self.target += delta;
    }
}

pub struct ProgressStore {
    pub name: String,
    pub maps: Vec<Map>,
    pub abilities: Vec<Ability>,
    pub progress: HashMap<(String, String, String, String), Target>,
}

impl ProgressStore {
    pub fn new(name: String) -> Self {
        ProgressStore {
            name: name,
            maps: Vec::new(),
            abilities: Vec::new(),
            progress: HashMap::new(),
        }
    }

    pub fn add_map(&mut self, m: Map) {
        self.maps.push(m);
    }

    pub fn add_zone(&mut self, map_sel: &Selector, zone_sel: &Selector) {
        let mo = map_sel.get_selected(&self.maps);
        if mo == None {
            return;
        }
        let m = mo.unwrap();
        let zo = zone_sel.get_selected(&m.zones);
        if zo == None {
            return;
        }
        let z = zo.unwrap();

        for a in &self.abilities {
            for u in &a.usages {
                self.progress.insert(
                    (
                        m.name.clone(),
                        z.name.clone(),
                        a.name.clone(),
                        u.name.clone(),
                    ),
                    Target::new(None, None),
                );
            }
        }
    }

    pub fn add_ability(&mut self, a: Ability) {
        self.abilities.push(a);
    }

    pub fn add_usage(&mut self, ability_sel: &Selector, usage_sel: &Selector) {
        let ao = ability_sel.get_selected(&self.abilities);
        if ao == None {
            return
        }
        let a = ao.unwrap();
        let uo = usage_sel.get_selected(&a.usages);
        if uo == None {
            return
        }
        let u = uo.unwrap();

        for m in &self.maps {
            for z in &m.zones {
                self.progress.insert(
                    (
                        m.name.clone(),
                        z.name.clone(),
                        a.name.clone(),
                        u.name.clone(),
                    ),
                    Target::new(None, None),
                );
            }
        }
    }

    pub fn get_target(&mut self, sel: &Selection) -> Option<&mut Target> {
        match sel {
            Selection {
                map: Some(msel),
                zone: Some(zsel),
                ability: Some(asel),
                usage: Some(usel),
            } => {
                // TODO: this assumes that the indices still match up!
                match (msel.get_selected(&self.maps), asel.get_selected(&self.abilities)) {
                    (Some(map), Some(ability)) => {
                        match (zsel.get_selected(&map.zones), usel.get_selected(&ability.usages)) {
                            (Some(zone), Some(usage)) => self.progress.get_mut(&(
                                map.name.clone(),
                                zone.name.clone(),
                                ability.name.clone(),
                                usage.name.clone(),
                            )),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
