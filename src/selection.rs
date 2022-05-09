use crate::model::Nameable;

#[derive(Debug)]
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

    // pub fn incr_targets(&mut self, p: ProgressStore) -> Result<()> {
    //     match (&self.map, &self.zone, &self.ability, &self.usage) {
    //         (Some(mname), Some(zname), Some(aname), Some(uname)) => {
    //             let m = p.get_map(mname);
    //             match p.get_target(m, z, a, u) {
    //                 Some(ref mut t) => Ok(()),
    //                 None => Err(Error::new(ErrorKind::Other, "Unknown combo of maps, zone, ability and utility? You shouldn't be seeing this.")),
    //             }
    //         },
    //         _ => Err(Error::new(ErrorKind::Other, "Must select a map, zone, ability and usage first!")),
    //     }
    // }
}

#[derive(Debug)]
pub enum Selector {
    Index(usize),
    Name(String),
}

impl Selector {
    pub fn new(s: String) -> Self {
        match s.parse::<usize>() {
            Ok(n) => Self::Index(n),
            Err(_) => Self::Name(s),
        }
    }

    pub fn get_selected<'a, S>(&self, vs: &'a Vec<S>) -> Option<&'a S>
    where
        S: Nameable,
        // T: SliceIndex<usize, Output=S> + IntoIterator<Item = S>,
    {
        match &self {
            Selector::Name(name) => {
                for u in vs {
                    if u.name() == name {
                        return Some(&u);
                    }
                }
                None
            }
            Selector::Index(idx) => vs.get(*idx)
        }
    }

    pub fn get_selected_mut<'a, S>(&self, vs: &'a mut Vec<S>) -> Option<&'a mut S> where S : Nameable {
        match &self {
            Selector::Name(name) => {
                for u in vs {
                    if u.name() == name {
                        return Some(u)
                    }
                }
                None
            }
            Selector::Index(idx) => vs.get_mut(*idx)
        }
    }
}
