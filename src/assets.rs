use tetra::{
    Context,
    graphics::{Texture, text::Font},
};

pub struct Assets {
    pub engine_texture: Texture,
    pub extractor_texture: Texture,
    pub gear4_texture: Texture,
    pub gear8_texture: Texture,
    pub gear12_texture: Texture,
    pub gear16_texture: Texture,
    pub power_texture: Texture,
    pub glooper_texture: Texture,
    pub gloop_texture: Texture,
    pub floor_texture: Texture,
    pub background_image: Texture,
    pub pixel: Texture,

    pub main_font: Font,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> Assets {
        Assets {
            gear4_texture: Texture::new(ctx, "./resources/gear4.png")
                .expect("Failed to get gear4 texture"),
            gear8_texture: Texture::new(ctx, "./resources/gear8.png")
                .expect("Failed to get gear8 texture"),
            gear12_texture: Texture::new(ctx, "./resources/gear12.png")
                .expect("Failed to get gear12 texture"),
            gear16_texture: Texture::new(ctx, "./resources/gear16.png")
                .expect("Failed to get gear16 texture"),
            extractor_texture: Texture::new(ctx, "./resources/extractor.png")
                .expect("Failed to get extractor texture"),
            engine_texture: Texture::new(ctx, "./resources/engine_base.png")
                .expect("Failed to load block texture"),
            power_texture: Texture::new(ctx, "./resources/power.png")
                .expect("Failed to load power button"),
            glooper_texture: Texture::new(ctx, "./resources/dodo.png")
                .expect("Failed to load power button"),
            gloop_texture: Texture::new(ctx, "./resources/gloop.png")
                .expect("Failed to load power button"),
            floor_texture: Texture::new(ctx, "./resources/floor.png")
                .expect("Failed to load floor"),
            background_image: Texture::new(ctx, "./resources/background_color_trees.png")
                .expect("Failed to load background image"),
            pixel: Texture::new(ctx, "./resources/pixel.png").expect("Failed to load pixel"),

            main_font: Font::vector(ctx, "./resources/slkscr.ttf", 16.)
                .expect("Failed to load main font"),
        }
    }
}

// Add button defaults here
//
