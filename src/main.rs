use rand_distr::{Distribution, Normal};
use std::f32::consts::PI;
use tetra::{
    Context, ContextBuilder, State,
    graphics::{self, Color, DrawParams, Rectangle, Texture, text::Text},
    input::{MouseButton, get_mouse_position, is_mouse_button_pressed},
    math::Vec2,
};

mod assets;
mod config;
mod entities;
use assets::*;
use config::*;
use entities::*;

// Move this to a config file/  sfcostruct?
const WIN_WIDTH: f32 = 1260.;
const WIN_HEIGHT: f32 = 720.;
const MAX_SMOG: f32 = 10000.;

struct GameState {
    assets: Assets,
    config: Config,
    engine: Engine, // find a way to store game items without adding them all here

    environment: Environment,
    extractor1: GloopExtractor,
    gloop_label: Text, // rotations: Text,
    smog_level: Text,
    start_engine: StartEngineButton,
    spare_gloop: f32,
    gloops: Vec<Gloop>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let assets = Assets::load(ctx);
        let cfg = Config::load(ctx);

        let environment = Environment::new();
        let engine = Engine::new(&assets);
        let start_button = StartEngineButton::new(&assets);

        let gloop_label = Text::new(
            format!("Gloop collected: {}, Gloop being added: {}", 0., 0.),
            assets.main_font.clone(),
        );
        let smog_level = Text::new(
            format!("Smog level: {}, Smog being added: {}", 0., 0.),
            assets.main_font.clone(),
        );

        let extractor = GloopExtractor::new(&assets);

        Ok(GameState {
            assets: assets,
            config: cfg,
            engine: engine,
            environment: environment,
            extractor1: extractor,
            gloop_label: gloop_label,
            smog_level: smog_level,
            start_engine: start_button,
            gloops: Vec::new(),
            spare_gloop: 0.,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mouse_pos = get_mouse_position(ctx);

        // Don't want to do this every time - how can it be done once or on a change?
        // Look into event function and what that means
        self.config.update_window_size(ctx);

        // reduced will change all? Should it be per gear?
        if self.engine.running && self.environment.gloop > 0. {
            // This currently uses overall gloop - should move it to be the gloop that has been put
            // into the engine (assuming infinite tank size? or should that be a limit too - pushes
            // to change from processing gloop as fuel for gloop as cash?)
            self.engine.gear.rotation_speed = 0.4;
        }

        if is_mouse_button_pressed(ctx, MouseButton::Left) {
            if self.engine.bounds(&self.config).contains_point(mouse_pos) {
                if self.engine.gear.rotation_speed == 0. {
                    self.engine.gear.rotation_speed += 0.5;
                } else {
                    println!(
                        "ADD LITTLE WARNING PERSON! HEALTH AND SAFETY - WATCH YOUR FINGERS ON SPINNING ENGINES"
                    );
                }
            }
            if self.start_engine.bounds().contains_point(mouse_pos) {
                self.engine.running = !self.engine.running;
            }
        }

        self.engine.gear.rotation_speed -= self.engine.friction; // friction at engine level - if
        if self.engine.gear.rotation_speed < 0. {
            self.engine.gear.rotation_speed = 0.;
        }

        self.environment.smog_level += self.engine.find_smog_output(self.environment.gloop);

        // 4 spins before gets to ~7300
        //if self.environment.smog_level >= 7500. {
        //    todo!("show prestige now");
        //   if self.environment.smog_level >= MAX_SMOG {
        //        todo!("Stop generating anymore gloop from the air now");
        //    }
        //}

        let gloop_collected = format!(
            "Gloop collected: {}, Gloop being added {}, air:gloop conversion: {}",
            self.environment.gloop,
            self.extractor1.base_extract_rate * (air_gloop_conversion(self.environment.smog_level)),
            air_gloop_conversion(self.environment.smog_level)
        );
        let smog_level = format!(
            "Smog level: {}, Smog being added: {}, smog pc: {}",
            self.environment.smog_level,
            self.engine.find_smog_output(self.environment.gloop),
            (self.environment.smog_level / MAX_SMOG) * 100.
        );

        self.gloop_label.set_content(gloop_collected);
        self.smog_level.set_content(smog_level);

        self.engine.gear.rotation += PI * self.engine.gear.rotation_speed;

        if self.extractor1.gear.rotation >= (2. * PI) {
            let gloop_to_drop = self.extractor1.base_extract_rate
                * air_gloop_conversion(self.environment.smog_level);
            self.environment.gloop += gloop_to_drop; // multiply by air quality factor
            self.extractor1.gear.rotation -= 2. * PI;

            self.spare_gloop += gloop_to_drop - (gloop_to_drop / 5.).floor() * 5.;
            let mut additional_gloop = 0.;
            if self.spare_gloop > 5. {
                additional_gloop += 1.;
                self.spare_gloop -= 5.;
            }

            println!(
                "Drop {} Gloops. There are {} gloops spare",
                (gloop_to_drop / 5.).floor() + additional_gloop,
                gloop_to_drop - (gloop_to_drop / 5.).floor() * 5.
            );

            self.extractor1.gloop_to_drop += (gloop_to_drop / 5.).floor() + additional_gloop;

            let mut rng = rand::rng();
            let spread = Normal::new(0., 4.).unwrap();

            if self.extractor1.gloop_to_drop > 0. {
                for _ in 1..=self.extractor1.gloop_to_drop as usize {
                    let y_change = spread.sample(&mut rng) as f32;
                    self.gloops.push(Gloop::new(
                        &self.assets,
                        Vec2::new(
                            self.config.window_width / 2. + y_change.floor() * 5.,
                            self.config.window_height - 512.,
                        ),
                    ));
                }
                self.extractor1.gloop_to_drop = 0.;

                println!("{:?}", self.gloops.len());
            }
        }

        for glp in &mut self.gloops {
            glp.update_position(ctx);
        }

        if self.engine.running && self.environment.gloop > 0. {
            if self.engine.gear.rotation >= (2. * PI) {
                self.environment.gloop -= self.engine.gloop_burned + self.engine.friction * 10.;
                self.engine.gear.rotation -= 2. * PI;
            }
        }
        self.environment.gloop = self.environment.gloop.max(0.); // starts very inefficient

        self.extractor1.gear.rotation += PI
            * (self.engine.gear.rotation_speed
                * (self.engine.gear.teeth / self.extractor1.gear.teeth));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        let mut reminders = Text::new(
            "Make smog much much worse - have a person press for you - maybe 2 then have them suggest turning it on. Once its on fill up with smog very fast and they say they remember why it wasn't on",
            self.assets.main_font.clone(),
        );

        reminders.draw(ctx, Vec2::new(16., 56.));
        self.gloop_label.draw(ctx, Vec2::new(16., 16.));
        self.smog_level.draw(ctx, Vec2::new(16., 36.));
        self.engine.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(56., self.config.window_height - 112.))
                .scale(Vec2::new(5., 5.))
                .origin(Vec2::new(
                    self.engine.texture.width() as f32 / 2.,
                    self.engine.texture.height() as f32 / 2.,
                )),
        );
        self.engine.gear.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(56., self.config.window_height - 112.))
                .rotation(self.engine.gear.rotation)
                .origin(Vec2::new(
                    self.engine.gear.texture.width() as f32 / 2.,
                    self.engine.gear.texture.height() as f32 / 2.,
                )),
        );

        let scale = 1. / (self.engine.gear.teeth / self.extractor1.gear.teeth);
        self.extractor1.gear.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(
                    self.config.window_width / 2.,
                    self.config.window_height - 512.,
                ))
                .scale(Vec2::new(scale, scale))
                .rotation(self.extractor1.gear.rotation)
                .origin(Vec2::new(
                    self.extractor1.gear.texture.width() as f32 / 2.,
                    self.extractor1.gear.texture.height() as f32 / 2.,
                )),
        );

        let mut red = 1.;
        let mut green = 0.;
        if self.engine.running {
            red = 0.;
            green = 1.;
        }
        self.start_engine.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(self.config.window_width - 156., 500.))
                .scale(Vec2::new(10., 10.))
                .color(Color::rgb(red, green, 0.)),
        );

        let floor = Texture::new(ctx, "./resources/floor.png")?;
        floor.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(0., self.config.window_height - 70.))
                .scale(Vec2::new(150., 5.)),
        );

        let mut pile_height = 0.;

        for glp in &self.gloops {
            glp.texture.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(glp.position.x, glp.position.y))
                    .scale(Vec2::new(0.5, 0.5))
                    .origin(Vec2::new(
                        glp.texture.width() as f32 / 2.,
                        glp.texture.height() as f32 / 2.,
                    )),
            );

            pile_height += 1.;
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("S.M.O.G", WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .fullscreen(true) // need to figure out how to look at edges for placement
        .build()?
        .run(GameState::new)
}

struct GloopExtractor {
    gear: Gear,
    base_extract_rate: f32,
    gloop_to_drop: f32,
    spare_gloop: f32,
}

impl GloopExtractor {
    fn new(assets: &Assets) -> GloopExtractor {
        GloopExtractor {
            gear: Gear::new(assets.extractor_texture.clone(), 20.),
            base_extract_rate: 35.,
            gloop_to_drop: 0.,
            spare_gloop: 0.,
        }
    }
}

///
struct Environment {
    gloop: f32,
    smog_level: f32,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            smog_level: 0.,
            gloop: 0.,
        }
    }
}

struct Gear {
    texture: Texture,
    rotation: f32,
    teeth: f32,
    rotation_speed: f32,
}

impl Gear {
    fn new(texture: Texture, teeth: f32) -> Gear {
        Gear {
            texture: texture,
            rotation: 0.,
            teeth: teeth,
            rotation_speed: 0.,
        }
    }
}

// Figure out how to make this more generic with a trait
struct StartEngineButton {
    texture: Texture,
    text: Text,
}

impl StartEngineButton {
    fn new(assets: &Assets) -> StartEngineButton {
        StartEngineButton {
            texture: assets.power_texture.clone(),
            text: Text::new("Start/Stop Engine", assets.main_font.clone()),
        }
    }
    fn bounds(&self) -> Rectangle {
        // hardcoded for now as position just put in above.
        Rectangle::new(
            WIN_WIDTH - 56. - (self.texture.width() as f32 * 5.) / 2.,
            WIN_HEIGHT - 112. - ((self.texture.height() as f32 * 5.) / 2.),
            self.texture.width() as f32 * 5.,
            self.texture.height() as f32 * 5.,
        )
    }
}

fn air_gloop_conversion(smog_level: f32) -> f32 {
    // 100. for percentage
    //
    let smog_pc = (smog_level / MAX_SMOG) * 100.;
    let conversion_rate = 115. - 0.1 * std::f32::consts::E.powf((smog_pc / 30.) + 4.);

    conversion_rate.min(100.).max(0.) / 100.
}
