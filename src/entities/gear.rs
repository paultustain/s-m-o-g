use tetra::{graphics::Texture, math::Vec2};

pub struct Gear {
    pub texture: Texture,
    pub rotation: f32,
    position: Vec2<f32>,
    pub teeth: f32,
    pub rotation_speed: f32,
}

impl Gear {
    pub fn new(texture: Texture, teeth: f32, position: Vec2<f32>) -> Gear {
        Gear {
            texture: texture,
            rotation: 0.,
            position: position,
            teeth: teeth,
            rotation_speed: 0.,
        }
    }

    pub fn get_width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn get_height(&self) -> f32 {
        self.texture.height() as f32
    }
}
