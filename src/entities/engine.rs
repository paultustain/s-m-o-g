use tetra::{
    graphics::{Rectangle, Texture},
    math::Vec2,
};

use crate::config::FLOOR_LEVEL;

use super::Gear;
const MAX_FUEL_LEVEL: f32 = 1000.;

#[derive(Clone, Copy)]
pub struct Engine {
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
    pub fn new() -> Engine {
        Engine {
            position: Vec2::new(185., FLOOR_LEVEL),
            fuel: 0.,
            running: false,
            gear: Gear::new(5.),
            efficiency: 0.,
            friction: 0.08,
            gloop_burned: 7., // per full spin
        }
    }

    pub fn get_width(&self, asset: &Texture) -> f32 {
        asset.width() as f32
    }

    pub fn get_height(&self, asset: &Texture) -> f32 {
        asset.height() as f32
    }

    pub fn bounds(&self, asset: &Texture) -> Rectangle {
        // hardcoded for now as position just put in above.
        //
        Rectangle::new(
            self.position.x,
            self.position.y - self.get_height(asset) as f32,
            self.get_width(asset) as f32,
            self.get_height(asset) as f32,
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
