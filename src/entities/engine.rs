use tetra::{
    Context,
    graphics::{Color, DrawParams, Rectangle, Texture},
    math::Vec2,
};

use crate::config::FLOOR_LEVEL;

use super::{Direction, Gear, HoverBox};
const MAX_FUEL_LEVEL: f32 = 1000.;

#[derive(Clone, Copy)]
pub struct Engine {
    pub position: Vec2<f32>,
    pub fuel: f32,
    pub running: bool,
    pub gear: Gear,
    pub efficiency: f32, // how much gloop is burned - move to be in the gear if this goes up, harder
    // to spin ( adds friction ) and therefore smog
    pub friction: f32, // how much the gear slows down after clicks / engine speed is slowed down
    pub gloop_burned: f32,
    pub hover_box: HoverBox,
}

impl Engine {
    pub fn new() -> tetra::Result<Engine> {
        let box_height = 20;

        Ok(Engine {
            position: Vec2::new(185., FLOOR_LEVEL),
            fuel: 0.,
            running: true,
            gear: Gear::new(4.),
            efficiency: 0.,
            friction: 0.08,
            gloop_burned: 7., // per full spin
            hover_box: HoverBox::new(
                50,
                box_height,
                Vec2::new(185. + 24., FLOOR_LEVEL - 48. - box_height as f32 - 5.),
                Direction::Up,
            )?,
        })
    }

    pub fn get_width(&self, asset: &Texture) -> f32 {
        asset.width() as f32
    }

    pub fn get_height(&self, asset: &Texture) -> f32 {
        asset.height() as f32
    }

    pub fn bounds(&self, asset: &Texture) -> Rectangle {
        // hardcoded for now as position just put in above.
        Rectangle::new(
            self.position.x,
            self.position.y - (self.get_height(asset) as f32),
            self.get_width(asset) as f32,
            self.get_height(asset) as f32,
        )
    }

    pub fn add_fuel(&mut self, amount: f32) {
        self.fuel += amount.min((MAX_FUEL_LEVEL - self.fuel).max(0.));
    }

    pub fn find_smog_output(&mut self, gloop: f32) -> f32 {
        // This needs to be much higher to make it not worth it at all
        if !self.running {
            return 0.;
        }

        // this should be gloop level within engine - not environment gloop
        if gloop == 0. {
            return 0.;
        }

        (self.gloop_burned + self.friction * 10.) * self.gear.rotation_speed * 0.5

        // this is per rotation - 0.3 should feel very
        // high at start due to inability to process gloop with new prestige can process gloop to
        // bring this and the amount burned down - make even higher!
    }

    pub fn is_full(&self) -> bool {
        self.fuel == MAX_FUEL_LEVEL
    }

    pub fn draw_feedback(&self, ctx: &mut Context, pixel: &Texture) {
        let on_off_pos = Vec2::new(self.position.x + 29., self.position.y - 25.);
        if self.running {
            pixel.draw(
                ctx,
                DrawParams::new().position(on_off_pos).color(Color::GREEN),
            );
        } else {
            pixel.draw(
                ctx,
                DrawParams::new().position(on_off_pos).color(Color::RED),
            );
        }

        let blocks = (5. * (self.fuel / MAX_FUEL_LEVEL)) as i8;

        for offset in 0..blocks {
            pixel.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(on_off_pos.x + offset as f32, on_off_pos.y - 2.))
                    .color(Color::RED),
            );
        }
    }
}
