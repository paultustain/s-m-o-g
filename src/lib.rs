/*
* use tetra::{Context, State, window};

mod assets;
mod scenes;

use assets::*;
use scenes::{main_scene::*, scene_manager::*};

pub struct GameState {
    assets: Assets,
    //background: Canvas,
    //background_image: Texture,
    scenes: Vec<Box<dyn Scene>>,
    scrolls: f32,
    //main_screen: Canvas, // move this to separate location
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let assets = Assets::load(ctx);
        //let background = Canvas::new(ctx, 1260, 720)?;
        //let canvas = Canvas::new(ctx, 1280, 720)?;
        let initial_scene = MainScene::new(ctx)?;
        //let background_img = assets.background_image.clone();

        Ok(GameState {
            assets: assets,
            //background: background,
            //background_image: background_img,
            //main_screen: canvas,
            scrolls: 0.,
            scenes: vec![Box::new(initial_scene)],
        })
    }

    pub fn update_score(&mut self) {
        self.scrolls += 1.;
    }
}
*/
