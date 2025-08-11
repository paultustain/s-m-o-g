use std::time::Duration;

use tetra::{
    Context,
    graphics::{DrawParams, Rectangle, Texture, animation::Animation},
    math::Vec2,
};

use crate::{config::FLOOR_LEVEL, environment::Environment, machinery::Machinery};

use super::{Engine, Gloop};

#[derive(Debug)]
pub struct Glooper {
    pub role: Role,
    pub position: Vec2<f32>,
    pub specialist: bool,
    pub scale: Vec2<f32>,
    pub animation: PlayerAnimations,
    // Split some of these into role details.
    resting: bool,
    time_since_hit: f32,

    pub gloop_collected: bool,
    pub gloop_held: Option<Gloop>, // Make this a vec when carrying more than one possible.
}

#[derive(Debug)]
pub enum Role {
    Hitter,
    Mover,
    Researcher,
    Idle,
}

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walking,
    Carrying,
}

#[derive(Debug)]
pub struct PlayerAnimations {
    state: PlayerState,
    idle: Animation,
    walking: Animation,
    carrying: Animation,
}

impl PlayerAnimations {
    fn new(ctx: &mut Context) -> tetra::Result<PlayerAnimations> {
        let texture = Texture::new(ctx, "./resources/movement_tilesheet.png")?;
        Ok(PlayerAnimations {
            state: PlayerState::Idle,
            idle: Animation::new(
                texture.clone(),
                Rectangle::row(0., 0., 16., 16.).take(2).collect(),
                Duration::from_secs_f64(0.2),
            ),
            walking: Animation::new(
                texture.clone(),
                Rectangle::row(0., 16., 16., 16.).take(4).collect(),
                Duration::from_secs_f64(0.1),
            ),

            carrying: Animation::new(
                texture,
                Rectangle::row(0., 32., 16., 16.).take(4).collect(),
                Duration::from_secs_f64(0.3),
            ),
        })
    }

    pub fn draw<P>(&self, ctx: &mut Context, params: P)
    where
        P: Into<DrawParams>,
    {
        self.current().draw(ctx, params)
    }

    fn current(&self) -> &Animation {
        match self.state {
            PlayerState::Idle => &self.idle,
            PlayerState::Walking => &self.walking,
            PlayerState::Carrying => &self.carrying,
        }
    }

    fn current_mut(&mut self) -> &mut Animation {
        match self.state {
            PlayerState::Idle => &mut self.idle,
            PlayerState::Walking => &mut self.walking,
            PlayerState::Carrying => &mut self.carrying,
        }
    }
    pub fn advance(&mut self, ctx: &Context) {
        self.current_mut().advance(ctx);
    }

    fn set_state(&mut self, state: PlayerState) {
        if self.state != state {
            self.state = state;
            self.current_mut().restart();
        }
    }
}

impl Glooper {
    pub fn new(ctx: &mut Context, pos: Vec2<f32>, role: Role) -> tetra::Result<Glooper> {
        Ok(Glooper {
            role: role,
            position: pos,
            specialist: false,
            scale: Vec2::new(1., 1.),
            animation: PlayerAnimations::new(ctx)?,
            resting: false,
            time_since_hit: 0.,
            gloop_collected: false,
            gloop_held: None,
        })
    }

    pub fn hit_engine(&mut self, machine: &mut Machinery) {
        if !self.resting {
            self.time_since_hit = 0.;
            self.position.x = machine.engine.position.x - 100.;
            self.resting = true;
            machine.turn_handle();
        } else {
            self.time_since_hit += 1.;
            if self.time_since_hit == 1. {
                self.position = Vec2::new(100., FLOOR_LEVEL)
            }
            if self.time_since_hit == 150. {
                self.resting = false;
            }
        }
    }

    pub fn move_gloop(&mut self, environment: &mut Environment, engine: &mut Engine) {
        if self.gloop_collected {
            self.gloop_held
                .as_mut()
                .unwrap()
                .carry_position(self.position);
            let move_speed = -0.4;
            self.position.x += move_speed;
            if self.position.x <= 240. {
                self.animation.set_state(PlayerState::Idle);
                self.scale = Vec2::new(1., 1.);
                if engine.is_full() {
                    environment.add_gloop(0);
                } else {
                    engine.add_fuel(self.gloop_held.as_ref().unwrap().value);
                }
                self.gloop_held = None;
                self.gloop_collected = false;
            }
        } else {
            if environment.loose_gloop == 0. {
                self.animation.set_state(PlayerState::Idle);
                return;
            }
            self.animation.set_state(PlayerState::Walking);
            let move_speed = 0.9;
            self.position.x += move_speed;
            if self.position.x
                >= ((environment.pile_locations.iter().max().unwrap() * 3) + 232) as f32
            {
                let pile = environment
                    .piles
                    .get_mut(environment.pile_locations.iter().max().unwrap())
                    .unwrap();

                let mut gloop = pile.pop().unwrap();
                if pile.len() == 0 {
                    environment
                        .empty_pile(*environment.pile_locations.iter().max().unwrap() as u128);
                }
                self.animation.set_state(PlayerState::Carrying);
                self.scale = Vec2::new(-1., 1.);
                gloop.carried = true;
                self.gloop_collected = true;
                environment.loose_gloop -= 1.;
                self.gloop_held = Some(gloop);
            }
        }
    }
}
