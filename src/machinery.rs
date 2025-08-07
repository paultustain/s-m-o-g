use std::f32::consts::PI;

use crate::{
    assets::Assets,
    entities::{Engine, Extractor},
    environment::Environment,
    utilities::random_pile,
};

pub struct Machinery {
    pub engine: Engine,
    pub extractors: Vec<Extractor>,
}

impl Machinery {
    pub fn new(assets: &Assets) -> Machinery {
        Machinery {
            engine: Engine::new(assets),
            extractors: vec![Extractor::new(assets)],
        }
    }

    pub fn turn_handle(&mut self) {
        if self.engine.gear.rotation_speed == 0. {
            self.engine.gear.rotation_speed = 0.5;
        }
    }

    pub fn move_machinery(&mut self, environment: &mut Environment) {
        self.engine.gear.rotation += PI * self.engine.gear.rotation_speed;

        for extractor in &mut self.extractors {
            let ratio = self.engine.gear.teeth / extractor.gear.teeth;
            let spin_amount = PI * self.engine.gear.rotation_speed * ratio;
            extractor.gear.rotation += spin_amount;

            if extractor.can_create_gloop(spin_amount) {
                let pile = random_pile();
                environment.add_gloop(pile);
            }
        }

        self.engine.gear.rotation_speed -= self.engine.friction;
        if self.engine.gear.rotation_speed < 0. {
            self.engine.gear.rotation_speed = 0.;
        }
    }
}
