use tetra::{
    graphics::{Rectangle, Texture},
    math::Vec2,
};

use crate::Assets;

pub struct Gloop {
    pub texture: Texture,
    value: f32,
    position: Vec2<f32>,
}

impl Gloop {
    pub fn new(assets: &Assets) -> Gloop {
        Gloop {
            texture: assets.gloop_texture.clone(),
            value: 5.,
            position: Vec2::new(0., 0.),
        }
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.texture.width() as f32,
            self.texture.height() as f32,
        )
    }
}
