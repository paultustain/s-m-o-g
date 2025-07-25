use tetra::{
    Context,
    graphics::{Texture, text::Font},
};

pub struct Assets {
    pub engine_texture: Texture,
    pub extractor_texture: Texture,
    pub gear_texture: Texture,
    pub power_texture: Texture,
    pub glooper_texture: Texture,
    pub gloop_texture: Texture,

    pub main_font: Font,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> Assets {
        Assets {
            gear_texture: Texture::new(ctx, "./resources/engine.png")
                .expect("Failed to get engine texture"),
            extractor_texture: Texture::new(ctx, "./resources/extractor.png")
                .expect("Failed to get extractor texture"),
            engine_texture: Texture::new(ctx, "./resources/block.png")
                .expect("Failed to load block texture"),
            power_texture: Texture::new(ctx, "./resources/power.png")
                .expect("Failed to load power button"),
            glooper_texture: Texture::new(ctx, "./resources/glooper.png")
                .expect("Failed to load power button"),
            gloop_texture: Texture::new(ctx, "./resources/gloop.png")
                .expect("Failed to load power button"),

            main_font: Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 16.)
                .expect("Failed to load main font"),
        }
    }
}
