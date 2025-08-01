use crate::{assets::Assets, entities::Gloop};

pub struct Environment {
    pub smog_level: f32,
    pub loose_gloop: Vec<Gloop>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            smog_level: 0.,
            loose_gloop: vec![],
        }
    }

    pub fn add_gloop(&mut self, assets: &Assets) {
        self.loose_gloop.push(Gloop::new(assets));
    }
}
