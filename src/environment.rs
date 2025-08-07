use std::collections::{HashMap, HashSet};

use crate::entities::Gloop;

type Column = u128;

pub struct Environment {
    pub smog_level: f32,
    pub piles: HashMap<Column, Vec<Gloop>>,
    pub loose_gloop: f32,
    pub pile_locations: HashSet<Column>, // hashmaps not ordered so using a vec to track.
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            smog_level: 0.,
            piles: HashMap::new(),
            loose_gloop: 0.,
            pile_locations: HashSet::new(),
        }
    }

    pub fn add_gloop(&mut self, pile: u128) {
        self.loose_gloop += 1.;
        match self.piles.get_mut(&pile) {
            Some(column) => {
                self.pile_locations.insert(pile);
                column.push(Gloop::new(pile));
            }
            None => {
                self.pile_locations.insert(pile);
                self.piles.insert(pile, vec![Gloop::new(pile)]);
            }
        }
    }

    pub fn empty_pile(&mut self, pile: u128) {
        self.pile_locations.remove(&pile);
    }
}
