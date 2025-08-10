use std::f32::consts::PI;

use tetra::math::Vec2;

use super::Gear;
#[derive(Clone, Copy)]
pub struct Extractor {
    pub gear: Gear,
    pub position: Vec2<f32>,
    base_extract_rate: f32,
    gloop_spare: f32,
}

impl Extractor {
    pub fn new() -> Extractor {
        Extractor {
            gear: Gear::new(20.),
            position: Vec2::new(300., 600.),
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
