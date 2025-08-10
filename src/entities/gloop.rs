use tetra::{
    Context,
    graphics::{Color, DrawParams},
    math::Vec2,
};

use crate::{assets::Assets, config::FLOOR_LEVEL};

const FALL_SPEED: f32 = 8.; // consider physics processes in space 

const GLOOP_GREY: Color = Color::rgba(100., 100., 100., 1.);

#[derive(Debug, Clone, Copy)]
pub struct Gloop {
    pub value: f32,
    pub pile_column: u128,
    pub moving: bool,
    pub position: Vec2<f32>,
    pub carried: bool,
}

impl Gloop {
    pub fn new(pile: u128) -> Gloop {
        Gloop {
            value: 5.,
            moving: true,
            pile_column: pile,
            position: Vec2::new(400., 400.),
            carried: false,
        }
    }

    pub fn draw(self, ctx: &mut Context, assets: &Assets) {
        // chance to make it more fall like by missing top corners when falling
        // Occasionally tweak colour?
        //G
        for w in -1..=1 {
            for h in -1..=1 {
                let pos = Vec2::new(self.position.x + w as f32, self.position.y + h as f32);
                assets
                    .pixel
                    .draw(ctx, DrawParams::new().position(pos).color(GLOOP_GREY));
            }
        }
    }

    pub fn carry_position(&mut self, position: Vec2<f32>) {
        self.position = position;
        self.position.y -= 16.;
    }

    pub fn update_position(&mut self, pile_height: u128) {
        // just moves straight to the pile
        // Add fall animation later
        self.position.x = ((self.pile_column * 3) + 235) as f32;
        self.position.y = FLOOR_LEVEL - (3 * (pile_height + 1)) as f32;
        self.moving = false;
        /*
        self.position.x += self.velocity.x * (FALL_SPEED + (self.dt * 0.015));
        self.position.y += self.velocity.y * (FALL_SPEED + (self.dt * 0.015));
        if self.position.y >= (720. - 70.) {
            self.position.y = 720. - 70.;
            self.moving = false;
            self.velocity = Vec2::zero();
        } else {
            self.dt += 1.;
            self.velocity.y += 0.05 * (1. - self.velocity.y);
            let mut multiplier = 1.;
            if self.velocity.x < 0. {
                multiplier = -1.
            }
            self.velocity.x = multiplier * (1. - pow(self.velocity.y, 2)).sqrt();
        }*/
    }
}
