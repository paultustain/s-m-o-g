use tetra::math::Vec2;

use super::Glooper;

const FALL_SPEED: f32 = 8.; // consider physics processes in space 

#[derive(Debug)]
pub struct Gloop {
    pub value: f32,
    dt: f32,
    pub pile_column: u128,
    pub moving: bool,
    // pub velocity: Vec2<f32>,
    pub position: Vec2<f32>,
    pub carried: bool,
}

impl Gloop {
    pub fn new(pile: u128) -> Gloop {
        Gloop {
            value: 5.,
            dt: 1.,
            moving: true,
            pile_column: pile,
            position: Vec2::new(400., 400.),
            carried: false,
        }
    }

    pub fn carry_position(&mut self, position: Vec2<f32>) {
        self.position = position;
        self.position.y -= 15.;
    }

    pub fn update_position(&mut self, pile_height: u128) {
        // just moves straight to the pile
        // Add fall animation later
        self.position.x = ((self.pile_column * 3) + 235) as f32;
        self.position.y = 700. - 50. - (3 * pile_height) as f32;
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

    /*
    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x - self.texture.width() as f32 / 2.,
            self.position.y - self.texture.width() as f32 / 2.,
            self.texture.width() as f32,
            self.texture.height() as f32,
        )
    }
    */
    /*
    fn can_fall(&self) -> bool {
        let floor = Rectangle::new(0., 720. - 70., 2000., 2.);
        let bounds = self.bounds();

        if bounds.intersects(&floor) {
            return false;
        }

        // find better way to sort this list so it doesn't have to straight loop through
        //for glp in gloops {
        //    if bounds != glp.bounds() {
        //       // dont check self
        //        if bounds.intersects(&glp.bounds()) {
        //            return false;
        //        }
        //    }
        //}

        return true;
    }*/
}
