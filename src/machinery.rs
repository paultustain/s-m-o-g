use std::f32::consts::PI;

use tetra::{Context, graphics::DrawParams, math::Vec2};

use crate::{
    assets::Assets,
    entities::{Engine, Extractor},
    environment::Environment,
    utilities::random_pile,
};

#[derive(Clone)]
pub struct Machinery {
    pub engine: Engine,
    pub extractors: Vec<Extractor>,
}

impl Machinery {
    pub fn new() -> Machinery {
        Machinery {
            engine: Engine::new(),
            extractors: vec![Extractor::new()],
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) {
        // origin bottom left corner to fit to floor
        assets.engine_texture.draw(
            ctx,
            DrawParams::new()
                .position(self.engine.position)
                .origin(Vec2::new(0., assets.engine_texture.height() as f32)),
        );

        assets.gear_texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(
                    self.engine.position.x + 14.,
                    self.engine.position.y - 14.,
                ))
                .origin(Vec2::new(
                    assets.gear_texture.width() as f32 / 2.,
                    assets.gear_texture.height() as f32 / 2.,
                ))
                .rotation(self.engine.gear.rotation),
        );

        for extractor in &self.extractors {
            // Very hardcoded! must fix
            assets.gear_texture.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(300., 600.))
                    .origin(Vec2::new(
                        assets.gear_texture.width() as f32 / 2.,
                        assets.gear_texture.height() as f32 / 2.,
                    ))
                    .rotation(extractor.gear.rotation)
                    .scale(Vec2::new(0.4, 0.4)),
            )
        }
    }

    pub fn turn_handle(&mut self) {
        if self.engine.gear.rotation_speed == 0. {
            self.engine.gear.rotation_speed = 0.5;
        }
    }

    pub fn move_machinery(&mut self, environment: &mut Environment) {
        self.engine.gear.rotation += PI * self.engine.gear.rotation_speed;
        for extractor in &mut self.extractors {
            let ratio = self.engine.gear.teeth / extractor.gear.teeth;
            let spin_amount = PI * self.engine.gear.rotation_speed * ratio;
            extractor.gear.rotation += spin_amount;

            if extractor.can_create_gloop(spin_amount) {
                let pile = random_pile();
                environment.add_gloop(pile);
            }
        }
        self.engine.gear.rotation_speed -= self.engine.friction;
        if self.engine.gear.rotation_speed < 0. {
            self.engine.gear.rotation_speed = 0.;
        }
    }
}
