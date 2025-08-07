use std::cmp;

use tetra::{
    graphics::{Rectangle, Texture},
    math::Vec2,
};

use crate::assets::Assets;

use super::Gear;
const MAX_FUEL_LEVEL: f32 = 1000.;

pub struct Engine {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub fuel: f32,
    pub running: bool,
    pub gear: Gear,
    pub efficiency: f32, // how much gloop is burned - move to be in the gear if this goes up, harder
    // to spin ( adds friction ) and therefore smog
    pub friction: f32, // how much the gear slows down after clicks / engine speed is slowed down
    pub gloop_burned: f32,
}

impl Engine {
    pub fn new(assets: &Assets) -> Engine {
        Engine {
            texture: assets.engine_texture.clone(),
            position: Vec2::new(
                256. - (assets.engine_texture.width() as f32 * 5.) / 2.,
                720. - 70. - (assets.engine_texture.height() as f32 * 1.5) / 2.,
            ),
            fuel: 0.,
            running: false,
            gear: Gear::new(assets.gear_texture.clone(), 5., Vec2::new(56., 720. - 70.)),
            efficiency: 0.,
            friction: 0.08,
            gloop_burned: 7., // per full spin
        }
    }

    pub fn get_width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn get_height(&self) -> f32 {
        self.texture.height() as f32
    }

    pub fn bounds(&self) -> Rectangle {
        // hardcoded for now as position just put in above.
        Rectangle::new(
            256. - (self.get_width() * 7.) / 2.,
            720. - 70. - (self.texture.height() as f32 * 1.5),
            self.texture.width() as f32 * 1.5,
            self.texture.height() as f32 * 1.5,
        )
    }

    pub fn add_fuel(&mut self, amount: f32) {
        self.fuel += amount.min((MAX_FUEL_LEVEL - self.fuel).max(0.));
    }

    pub fn find_smog_output(&mut self, gloop: f32) -> f32 {
        // This needs to be much higher to make it not worth it at all
        if !self.running {
            return 0.;
        }

        // this should be gloop level within engine - not environment gloop
        if gloop == 0. {
            return 0.;
        }

        (self.gloop_burned + self.friction * 10.) * self.gear.rotation_speed * 0.5

        // this is per rotation - 0.3 should feel very
        // high at start due to inability to process gloop with new prestige can process gloop to
        // bring this and the amount burned down - make even higher!
    }

    pub fn is_full(&self) -> bool {
        self.fuel == MAX_FUEL_LEVEL
    }
}
