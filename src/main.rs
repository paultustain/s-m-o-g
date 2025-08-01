use scenes::{main_scene::MainScene, scene_manager::SceneManager};
use tetra::ContextBuilder;

pub mod assets;
pub mod config;
pub mod entities;
pub mod environment;
pub mod machinery;
mod scenes;

fn main() -> tetra::Result {
    ContextBuilder::new("smog", 1260, 720)
        .quit_on_escape(true)
        // .fullscreen(true)
        .show_mouse(true)
        .build()?
        .run(|ctx| {
            let scene = MainScene::new(ctx)?;
            Ok(SceneManager::new(Box::new(scene)))
        })
}
