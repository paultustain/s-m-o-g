use tetra::{
    Context,
    graphics::{Color, DrawParams},
    math::Vec2,
};

use crate::assets::Assets;

const EDGE_COLOUR: Color = Color::rgba(0., 75.3, 68.7, 1.);

#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn is_corner(&self, main_box: &MainBox) -> bool {
        if *self == Point::new(-main_box.height, -main_box.width) {
            return true;
        }
        if *self == Point::new(main_box.height, -main_box.width) {
            return true;
        }
        if *self == Point::new(-main_box.height, main_box.width) {
            return true;
        }
        if *self == Point::new(main_box.height, main_box.width) {
            return true;
        }

        false
    }

    fn is_edge(&self, main_box: &MainBox) -> bool {
        if self.x == -main_box.width {
            return true;
        }
        if self.x == main_box.width {
            return true;
        }
        if self.y == -main_box.height {
            return true;
        }
        if self.y == main_box.height {
            return true;
        }

        false
    }
}

pub struct MainBox {
    // This will be a black button when you hover over something
    // Some will have info, some will have resouces
    width: i32,
    height: i32,
    origin: Vec2<f32>,
}

impl MainBox {
    pub fn new(half_width: u32, half_height: u32, origin: Vec2<f32>) -> MainBox {
        MainBox {
            width: half_width as i32,
            height: half_height as i32,
            origin: origin,
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) {
        for w in -self.width..=self.width {
            for h in -self.height..=self.height {
                let p = Point::new(w, h);
                if !p.is_corner(self) {
                    let pos = Vec2::new(self.origin.x + w as f32, self.origin.y + h as f32);
                    let mut color = Color::BLACK;
                    if p.is_edge(self) {
                        color = EDGE_COLOUR
                    }

                    assets
                        .pixel
                        .draw(ctx, DrawParams::new().position(pos).color(color))
                }
            }
        }
    }
}
pub struct SecondaryButton {}
