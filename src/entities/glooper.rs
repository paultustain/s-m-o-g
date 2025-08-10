use tetra::math::Vec2;

use crate::{config::FLOOR_LEVEL, environment::Environment, machinery::Machinery};

use super::{Engine, Gloop};

#[derive(Debug)]
pub struct Glooper {
    pub role: Role,
    pub position: Vec2<f32>,
    pub specialist: bool,
    pub scale: Vec2<f32>,
    // Split some of these into role details.
    resting: bool,
    bounce_up: bool,
    bounce_level: f32,
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

impl Glooper {
    pub fn new(pos: Vec2<f32>, role: Role) -> Glooper {
        Glooper {
            role: role,
            position: pos,
            specialist: false,
            scale: Vec2::new(1., 1.),
            resting: false,
            bounce_up: true,
            bounce_level: 0.,
            time_since_hit: 0.,
            gloop_collected: false,
            gloop_held: None,
        }
    }

    pub fn bounce(&mut self) {
        if self.bounce_up {
            self.position.y += 0.4 as f32;
            self.bounce_level += 0.4;
            if self.bounce_level >= 4. {
                self.bounce_up = false;
            }
        } else {
            self.position.y -= 0.4 as f32;
            self.bounce_level -= 0.4;
            if self.bounce_level <= 0. {
                self.bounce_up = true;
            }
        }
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
                self.scale = Vec2::new(1., 1.);
                engine.add_fuel(self.gloop_held.as_ref().unwrap().value);
                self.gloop_held = None;
                self.gloop_collected = false;
            }
        } else {
            if environment.loose_gloop == 0. {
                return;
            }
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
                self.scale = Vec2::new(-1., 1.);
                gloop.carried = true;
                self.gloop_collected = true;
                environment.loose_gloop -= 1.;
                self.gloop_held = Some(gloop);
            }
        }
    }
}
