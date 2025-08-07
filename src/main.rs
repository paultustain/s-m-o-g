use scenes::{main_scene::MainScene, scene_manager::SceneManager};
use tetra::ContextBuilder;

pub mod assets;
pub mod config;
pub mod entities;
pub mod environment;
pub mod machinery;
pub mod office;
pub mod utilities;

mod scenes;

// This is a test all before main can be removed.
/*
use rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};*/

/*
struct Gloop {
    position: Vec2<f32>,
}

impl Gloop {
    fn new(assets: &Assets, position: Vec2<f32>) -> Gloop {
        Gloop { position: position }
    }
}

struct Physics {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,

    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhaseBvh,
    narrow_phase: NarrowPhase,
    joints: ImpulseJointSet,
    multi_join: MultibodyJointSet,
    ccd: CCDSolver,
}

impl Physics {
    fn new() -> Physics {
        let mut im = IntegrationParameters::default();
        im.max_ccd_substeps = 25;

        Physics {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0., 250.),
            integration_parameters: im,
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joints: ImpulseJointSet::new(),
            multi_join: MultibodyJointSet::new(),
            ccd: CCDSolver::new(),
        }
    }
}

struct Environment {
    texture: Texture,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    loose_gloop: Vec<Gloop>,
}

impl Environment {
    fn new(assets: &Assets) -> Environment {
        let position = Vector2::new(630., 650.);

        let mut rbs = RigidBodySet::new();
        let floor = RigidBodyBuilder::new(RigidBodyType::Fixed)
            .position(Isometry2::new(position, 0.))
            .build();

        let floor_handler = rbs.insert(floor);

        let mut cs = ColliderSet::new();
        let collider = ColliderBuilder::cuboid(630., 2.5).build();
        cs.insert_with_parent(collider, floor_handler, &mut rbs);

        Environment {
            texture: assets.floor_texture.clone(),
            bodies: rbs,
            colliders: cs,
            loose_gloop: Vec::new(),
        }
    }

    fn create_block(&mut self, ctx: &mut Context, assets: &Assets) {
        let mouse_pos = input::get_mouse_position(ctx);

        let block = RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .position(Isometry2::new(Vector2::new(mouse_pos.x, mouse_pos.y), 0.))
            .user_data(self.loose_gloop.len() as u128 + 1)
            .ccd_enabled(false)
            .build();

        let gloop_handle = self.bodies.insert(block);
        let collider = ColliderBuilder::cuboid(
            assets.gloop_texture.width() as f32 / 2.,
            assets.gloop_texture.height() as f32 / 2.,
        )
        .density(22.)
        .build();

        self.colliders
            .insert_with_parent(collider, gloop_handle, &mut self.bodies);
    }
}

struct GameState {
    assets: Assets,
    physics: Physics,
    environment: Environment,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let assets = Assets::load(ctx);

        Ok(GameState {
            assets: Assets::load(ctx),
            physics: Physics::new(),
            environment: Environment::new(&assets),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut tetra::Context) -> tetra::Result {
        let physics_hook = ();
        let event_handler = ();

        self.physics.pipeline.step(
            &self.physics.gravity,
            &self.physics.integration_parameters,
            &mut self.physics.island_manager,
            &mut self.physics.broad_phase,
            &mut self.physics.narrow_phase,
            &mut self.environment.bodies,
            &mut self.environment.colliders,
            &mut self.physics.joints,
            &mut self.physics.multi_join,
            &mut self.physics.ccd,
            &physics_hook,
            &event_handler,
        );

        if input::is_mouse_button_pressed(ctx, input::MouseButton::Left) {
            self.environment.create_block(ctx, &self.assets);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut tetra::Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        self.environment.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(0., 650.))
                .scale(Vec2::new(200., 1.)),
        );

        for (handle, body) in self.environment.bodies.iter() {
            let pos = body.position();

            // println!("{:#?}", pos.translation.x);

            self.assets.gloop_texture.draw(
                ctx,
                DrawParams::new().position(Vec2::new(pos.translation.x, pos.translation.y)),
            )
        }
        Ok(())
    }
}
*/

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
// put this back into run
//|ctx| {
//let scene = MainScene::new(ctx)?;
//Ok(SceneManager::new(Box::new(scene)))
//}
