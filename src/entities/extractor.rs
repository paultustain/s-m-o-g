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

    pub fn generate_gloop(&mut self) {}
}
