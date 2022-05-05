use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

    fn add_zone(&mut self, z: Zone) {
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

#[derive(Eq, Hash, PartialEq)]
pub struct Zone {
    pub name: String,
}

impl Zone {
    pub fn new(name: String) -> Self {
        Zone { name: name }
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

    fn add_usage(&mut self, u: Usage) {
        self.usages.push(u);
    }
}

impl PartialEq for Ability {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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

    pub fn new_map(&mut self, m: Map) {
        self.maps.push(m);
    }

    pub fn new_zone(&mut self, m: &mut Map, z: Zone) {
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
        m.add_zone(z);
    }

    pub fn new_ability(&mut self, a: Ability) {
        self.abilities.push(a);
    }

    pub fn new_usage(&mut self, a: &mut Ability, u: Usage) {
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
        a.add_usage(u);
    }

    pub fn get_target(&mut self, m: &String, z: &String, a: &String, u: &String) -> Option<&mut Target> {
        self.progress.get_mut(&(
                m.clone(), z.clone(), a.clone(), u.clone()
        ))
    }
}
