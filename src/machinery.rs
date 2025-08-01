use std::f32::consts::PI;

use crate::{
    assets::Assets,
    entities::{Engine, Extractor},
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
            self.engine.gear.rotation_speed = 0.75;
        }
    }

    pub fn move_machinery(&mut self) {
        self.engine.gear.rotation += PI * self.engine.gear.rotation_speed;

        for extractor in &mut self.extractors {
            let ratio = self.engine.gear.teeth / extractor.gear.teeth;
            extractor.gear.rotation += PI * self.engine.gear.rotation_speed * ratio;
        }

        self.engine.gear.rotation_speed -= self.engine.friction;
        if self.engine.gear.rotation_speed < 0. {
            self.engine.gear.rotation_speed = 0.;
        }
    }
}
