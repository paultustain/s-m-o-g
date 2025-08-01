use rand_distr::{Distribution, Normal};
use tetra::{
    Context,
    graphics::{Rectangle, Texture},
    math::Vec2,
    window,
};

use crate::assets::Assets;

pub struct Gloop {
    pub texture: Texture,
    value: f32,
    pub position: Vec2<f32>,
}

impl Gloop {
    pub fn new(assets: &Assets) -> Gloop {
        Gloop {
            texture: assets.gloop_texture.clone(),
            value: 5.,
            position: random_start_position(),
        }
    }

    pub fn update_position(&mut self, ctx: &Context, gloops: &Vec<Gloop>) {
        // currently only going downwards - maybe add end position to move to in gloop?
        // consider moving struct / enum that contains final position
        // would be more grid based
        if self.can_fall(ctx, gloops) {
            self.position.y += 10.;
        }
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.texture.width() as f32 * 0.5,
            self.texture.height() as f32 * 0.5,
        )
    }

    fn can_fall(&self, ctx: &Context, gloops: &Vec<Gloop>) -> bool {
        let floor = Rectangle::new(0., window::get_height(ctx) as f32 - 70., 2000., 2.);
        let bounds = self.bounds();

        if bounds.intersects(&floor) {
            return false;
        }

        // find better way to sort this list so it doesn't have to straight loop through
        //for glp in gloops {
        //    if bounds != glp.bounds() {
        //       // dont check self
        //        if bounds.intersects(&glp.bounds()) {
        //            return false;
        //        }
        //    }
        //}

        return true;
    }
}
fn random_start_position() -> Vec2<f32> {
    let mut rng = rand::rng();
    let spread = Normal::new(0., 40.).unwrap();
    let dist = spread.sample(&mut rng) as f32;

    Vec2::new(400. + dist, 500.)
}
