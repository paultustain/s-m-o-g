use tetra::{graphics::Texture, math::Vec2};

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
            teeth: teeth,
            rotation_speed: 0.,
        }
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
