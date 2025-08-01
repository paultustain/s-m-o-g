use tetra::{
    Context,
    graphics::{DrawParams, Texture},
    math::Vec2,
    window,
};

use crate::assets::Assets;

pub struct Glooper {
    pub texture: Texture,
    pub role: Role,
    pub position: Vec2<f32>,
    pub scale: f32,
}

enum Role {
    Hitter,
    Mover,
    Researcher,
    Idle,
}

impl Glooper {
    pub fn new(assets: &Assets, pos: Vec2<f32>) -> Glooper {
        Glooper {
            texture: assets.glooper_texture.clone(),
            role: Role::Idle,
            position: pos,
            scale: 0.75,
        }
    }
}
