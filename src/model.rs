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

    pub fn add_zone(&mut self, z:Zone) {
        self.zones.push(z)
    }
}

pub struct Zone {
    pub name: String,
}

impl Zone {
    pub fn new(name: String) -> Self {
        Zone {
            name: name,
        }
    }
}

pub struct Ability {
    pub name: String,
    pub usages: Vec<Usage>
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

pub struct Usage {
    pub name: String
}

impl Usage {
    pub fn new(name: String) -> Usage {
        Usage {
            name: name
        }
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
