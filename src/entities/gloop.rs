use tetra::{
    Context,
    graphics::{Rectangle, Texture},
    math::Vec2,
    window,
};

use crate::Assets;

pub struct Gloop {
    pub texture: Texture,
    value: f32,
    pub position: Vec2<f32>,
}

impl Gloop {
    pub fn new(assets: &Assets, pos: Vec2<f32>) -> Gloop {
        Gloop {
            texture: assets.gloop_texture.clone(),
            value: 5.,
            position: pos,
        }
    }

    pub fn update_position(&mut self, ctx: &Context) {
        if self.can_fall(ctx) {
            self.position.y += 10.;
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

    fn can_fall(&self, ctx: &Context) -> bool {
        let floor = Rectangle::new(0., window::get_height(ctx) as f32 - 60., 2000., 2.);
        let bounds = self.bounds();

        if bounds.intersects(&floor) {
            return false;
        }

        return true;
    }
}
