use crate::entities::{Glooper, Role};

pub struct Office {
    pub hitters: Vec<Glooper>,
    pub idlers: Vec<Glooper>,
    pub movers: Vec<Glooper>,
}

impl Office {
    pub fn new() -> Office {
        Office {
            hitters: Vec::new(),
            idlers: Vec::new(),
            movers: Vec::new(),
        }
    }

    pub fn add_glooper(&mut self, glooper: Glooper) {
        match glooper.role {
            Role::Hitter => self.hitters.push(glooper),
            Role::Mover => self.movers.push(glooper),
            Role::Researcher => todo!("Add researcher list"),
            Role::Idle => self.idlers.push(glooper),
        }
    }
}
