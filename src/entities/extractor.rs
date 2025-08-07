use std::f32::consts::PI;

use tetra::math::Vec2;

use crate::assets::Assets;

use super::Gear;

pub struct Extractor {
    pub gear: Gear,
    base_extract_rate: f32,
    gloop_spare: f32,
}

impl Extractor {
    pub fn new(assets: &Assets) -> Extractor {
        Extractor {
            gear: Gear::new(assets.gear_texture.clone(), 20., Vec2::new(800., 300.)),
            base_extract_rate: 35.,
            gloop_spare: 0.,
        }
    }

    pub fn can_create_gloop(&mut self, spin_amount: f32) -> bool {
        let gloop_earned = self.base_extract_rate / (2. * PI);

        self.gloop_spare += gloop_earned * spin_amount;

        // 5. should be gloop value const
        if self.gloop_spare > 5. {
            self.gloop_spare -= 5.;
            return true;
        }

        false
    }
}
