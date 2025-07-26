use tetra::graphics::{Rectangle, Texture};

use crate::{Gear, assets::Assets, config::Config};

pub struct Engine {
    pub texture: Texture,
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
            running: false,
            gear: Gear::new(assets.gear_texture.clone(), 5.),
            efficiency: 0.,
            friction: 0.08,
            gloop_burned: 7., // per full spin
        }
    }

    pub fn bounds(&self, cfg: &Config) -> Rectangle {
        // hardcoded for now as position just put in above.
        Rectangle::new(
            56. - (self.texture.width() as f32 * 5.) / 2.,
            cfg.window_height - 112. - ((self.texture.height() as f32 * 5.) / 2.),
            self.texture.width() as f32 * 5.,
            self.texture.height() as f32 * 5.,
        )
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

        (self.gloop_burned + self.friction * 10.) * self.gear.rotation_speed * 0.5 // this is per rotation - 0.3 should feel very
        //
        // high at start due to inability to process gloop with new prestige can process gloop to
        // bring this and the amount burned down
    }
}
