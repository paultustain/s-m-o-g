use tetra::{Context, window};

pub struct Config {
    pub window_height: f32,
    pub window_width: f32,
}

impl Config {
    pub fn load(ctx: &Context) -> Config {
        println!("{} {}", window::get_height(ctx), window::get_width(ctx));
        Config {
            window_height: window::get_height(ctx) as f32,
            window_width: window::get_width(ctx) as f32,
        }
    }

    pub fn update_window_size(&mut self, ctx: &Context) {
        self.window_height = window::get_height(ctx) as f32;
        self.window_width = window::get_width(ctx) as f32;
    }
}
