use tetra::{
    Context,
    graphics::{Color, DrawParams},
    math::Vec2,
};

use crate::assets::Assets;

const TOOTH_COLOUR: Color = Color::rgb(0., 0., 0.);

#[derive(Clone, Copy)]
pub struct Gear {
    pub rotation: f32,
    pub teeth: f32,
    pub rotation_speed: f32,
}

impl Gear {
    pub fn new(teeth: f32) -> Gear {
        Gear {
            rotation: 0.,
            // teeth can be 4, 8, 12, 16 on small base
            teeth: teeth,
            rotation_speed: 0.,
        }
    }

    pub fn draw(&self, ctx: &mut Context, pos: Vec2<f32>, assets: &Assets) {
        let asset = match self.teeth {
            4. => assets.gear4_texture.clone(),
            8. => assets.gear8_texture.clone(),
            12. => assets.gear12_texture.clone(),
            16.0..=100. => assets.gear16_texture.clone(),
            _ => todo!("Bigger needed"),
        };
        let origin = Vec2::new(asset.width() as f32 / 2., asset.height() as f32 / 2.);

        asset.draw(
            ctx,
            DrawParams::new()
                .position(pos)
                .origin(origin)
                .rotation(self.rotation),
        );
    }

    /*
    pub fn get_width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn get_height(&self) -> f32 {
        self.texture.height() as f32
    }
    */
}
