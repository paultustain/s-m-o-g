use tetra::{Context, window};

pub const ZOOM_SPEED: f32 = 0.01;
pub const MOVEMENT_SPEED: f32 = 4.;
pub const MIN_CAMERA_SCALE: f32 = 0.5;
pub const MAX_CAMERA_SCALE: f32 = 3.;
/// Any general configuration about the file
/// Add all the parameters that are used on game load here
pub struct Config {
    pub window_height: f32,
    pub window_width: f32,
    pub show_mouse: bool, // etc...
}

impl Config {
    pub fn load(ctx: &Context) -> Config {
        println!("{} {}", window::get_height(ctx), window::get_width(ctx));
        Config {
            window_height: window::get_height(ctx) as f32,
            window_width: window::get_width(ctx) as f32,
            show_mouse: true,
        }
    }

    pub fn update_window_size(&mut self, ctx: &Context) {
        self.window_height = window::get_height(ctx) as f32;
        self.window_width = window::get_width(ctx) as f32;
    }
}
