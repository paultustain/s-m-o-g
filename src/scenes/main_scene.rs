use tetra::{
    Context,
    graphics::{self, Camera, Canvas, Color, DrawParams, text::Text},
    input::{self, Key, MouseButton},
    math::Vec2,
};

use crate::{
    assets::Assets,
    config::{MAX_CAMERA_SCALE, MIN_CAMERA_SCALE, MOVEMENT_SPEED, ZOOM_SPEED},
    environment::Environment,
    machinery::Machinery,
};

use super::scene_manager::{Scene, Transition};

pub struct MainScene {
    background: Canvas,
    canvas: Canvas,
    camera: Camera,

    assets: Assets,
    machinery: Machinery,
    environment: Environment,
}

impl MainScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<MainScene> {
        let mut cam = Camera::new(1260., 720.);
        cam.position = Vec2::new(630., 360.);
        Ok(MainScene {
            background: Canvas::new(ctx, 1260, 720)?,
            canvas: Canvas::new(ctx, 1260, 720)?,
            camera: cam,
            assets: Assets::load(ctx),
            machinery: Machinery::new(&Assets::load(ctx)),
            environment: Environment::new(),
        })
    }
}

impl Scene for MainScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let mouse_pos = self.camera.mouse_position(ctx);

        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            if self.machinery.engine.bounds().contains_point(mouse_pos) {
                self.machinery.turn_handle();
            }
        }

        self.machinery.move_machinery();

        // Move camera movement? put camera consts in that file
        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_mouse_scrolled_up(ctx) {
            if self.camera.scale.x <= MAX_CAMERA_SCALE {
                self.camera.scale += ZOOM_SPEED;
            }
        }
        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_mouse_scrolled_down(ctx) {
            if self.camera.scale.x >= MIN_CAMERA_SCALE {
                self.camera.scale -= ZOOM_SPEED;
            }
        }

        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_key_down(ctx, Key::Up) {
            self.camera.position.y -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_key_down(ctx, Key::Down) {
            self.camera.position.y += MOVEMENT_SPEED;
        }
        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_key_down(ctx, Key::Left) {
            self.camera.position.x -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_key_down(ctx, Key::Right) {
            self.camera.position.x += MOVEMENT_SPEED;
        }

        self.camera.update();

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::set_canvas(ctx, &self.background);

        self.assets.background_image.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::zero())
                .origin(Vec2::zero())
                .scale(Vec2::new(7., 3.)),
        );
        graphics::reset_canvas(ctx);

        graphics::set_canvas(ctx, &self.canvas);

        graphics::clear(ctx, Color::rgba(1., 1., 1., 0.));
        let mut label = Text::new("Testing", self.assets.main_font.clone());
        label.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(16., 16.))
                .color(Color::BLUE),
        );

        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        self.machinery.engine.texture.draw(
            ctx,
            DrawParams::new()
                .origin(Vec2::new(
                    self.machinery.engine.get_width() / 2.,
                    self.machinery.engine.get_height() / 2.,
                ))
                .position(self.machinery.engine.position)
                .scale(Vec2::new(5., 5.)),
        );

        self.machinery.engine.gear.texture.draw(
            ctx,
            DrawParams::new()
                .origin(Vec2::new(
                    self.machinery.engine.gear.get_width() / 2.,
                    self.machinery.engine.gear.get_height() / 2.,
                ))
                .position(Vec2::new(
                    256. - 8. - (self.machinery.engine.gear.get_width() / 2.),
                    720. - 78. - (self.machinery.engine.gear.get_height() / 2.),
                ))
                .rotation(self.machinery.engine.gear.rotation),
        );

        for extractor in &self.machinery.extractors {
            extractor.gear.texture.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(400., 500.))
                    .origin(Vec2::new(
                        extractor.gear.get_width() / 2.,
                        extractor.gear.get_height() / 2.,
                    ))
                    .rotation(extractor.gear.rotation),
            );
        }

        self.assets.floor_texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(0., 720. - 70.))
                .scale(Vec2::new(150., 5.)),
        );

        graphics::reset_transform_matrix(ctx);

        graphics::reset_canvas(ctx);

        self.background.draw(ctx, DrawParams::new());
        self.canvas.draw(ctx, DrawParams::new());

        // self.scaler.draw(ctx);

        Ok(Transition::None)
    }
}
