use std::f32::consts::PI;

use tetra::{
    Context,
    graphics::{Color, DrawParams},
    math::Vec2,
};

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
    pub fn new() -> tetra::Result<Machinery> {
        Ok(Machinery {
            engine: Engine::new()?,
            extractors: vec![Extractor::new()],
        })
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) {
        // origin bottom left corner to fit to floor
        assets.engine_texture.draw(
            ctx,
            DrawParams::new()
                .position(self.engine.position)
                .origin(Vec2::new(0., assets.engine_texture.height() as f32)),
        );

        if self.engine.hover_box.showing {
            self.engine.hover_box.draw(ctx, assets);
        }

        self.engine.gear.draw(
            ctx,
            Vec2::new(self.engine.position.x + 14., self.engine.position.y - 14.),
            assets,
        );

        self.engine.draw_feedback(ctx, &assets.pixel);
        for extractor in &self.extractors {
            // Very hardcoded! must fix
            extractor.gear.draw(ctx, Vec2::new(300., 600.), assets)
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
