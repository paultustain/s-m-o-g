use tetra::{
    Context,
    graphics::{self, Camera, Canvas, Color, DrawParams, text::Text},
    input::{self, Key, MouseButton},
    math::{Rect, Vec2},
};

use crate::{
    assets::Assets,
    config::{FLOOR_LEVEL, MAX_CAMERA_SCALE, MIN_CAMERA_SCALE, MOVEMENT_SPEED, ZOOM_SPEED},
    entities::{Glooper, MainBox, Role},
    environment::Environment,
    machinery::Machinery,
    office::Office,
};

use super::scene_manager::{Scene, Transition};

pub struct MainScene {
    background: Canvas,
    canvas: Canvas,
    camera: Camera,

    assets: Assets,
    machinery: Machinery,
    environment: Environment,
    office: Office,
    button: MainBox, // Create a UI file at higher level?
}

impl MainScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<MainScene> {
        let assets = Assets::load(ctx);
        let mut cam = Camera::new(1260., 720.);
        cam.position = Vec2::new(630., 360.);
        cam.scale = Vec2::new(3., 3.);
        cam.position = Vec2::new(200., FLOOR_LEVEL - 115.);

        let mut office = Office::new();
        let glooper = Glooper::new(ctx, Vec2::new(100., FLOOR_LEVEL), Role::Idle);
        office.add_glooper(glooper?);
        let mover = Glooper::new(ctx, Vec2::new(100., FLOOR_LEVEL), Role::Mover);
        office.add_glooper(mover?);

        Ok(MainScene {
            background: Canvas::new(ctx, 1260, 720)?,
            canvas: Canvas::new(ctx, 1260, 720)?,
            camera: cam,
            assets: assets,
            machinery: Machinery::new()?,
            environment: Environment::new(),
            office: office,
            button: MainBox::new(50, 30, Vec2::new(630., 50.)),
        })
    }
}

impl Scene for MainScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let mouse_pos = self.camera.mouse_position(ctx);
        let mut column_top = Rect::new(0., 0., 1., 1.);

        if self
            .machinery
            .engine
            .bounds(&self.assets.engine_texture)
            .contains_point(mouse_pos)
        {
            self.machinery.engine.hover_box.showing = true;

            // self.machinery.turn_handle();
        } else {
            if self.machinery.engine.hover_box.showing {
                if self
                    .machinery
                    .engine
                    .hover_box
                    .bounds()
                    .contains_point(mouse_pos)
                {
                    self.machinery.engine.hover_box.showing = true;
                } else {
                    self.machinery.engine.hover_box.showing = false;
                }
            } else {
                self.machinery.engine.hover_box.showing = false;
            }
        }

        if input::is_mouse_button_pressed(ctx, MouseButton::Left)
            || input::is_mouse_button_down(ctx, MouseButton::Left)
        {
            if mouse_pos.x > 232. {
                let column_hover = (mouse_pos.x - 235.) / 3.;
                if self.environment.piles.contains_key(&(column_hover as u128)) {
                    let column_height = self.environment.piles[&(column_hover as u128)].len() + 1;
                    column_top = Rect::new(
                        (column_hover as f32 * 3.) + 232.,
                        FLOOR_LEVEL - (3. * (column_height - 1) as f32),
                        3.,
                        3.,
                    );
                }

                if column_top.contains_point(mouse_pos) {
                    let pile = self
                        .environment
                        .piles
                        .get_mut(&(column_hover as u128))
                        .unwrap();

                    match pile.pop() {
                        Some(gloop) => {
                            if pile.len() == 0 {
                                self.environment.empty_pile(column_hover as u128);
                            }
                            if gloop.pile_column <= 3 {
                                if self.machinery.engine.is_full() {
                                    self.environment.add_gloop(0);
                                } else {
                                    self.machinery.engine.add_fuel(gloop.value);
                                    self.environment.loose_gloop -= 1.;
                                }
                            } else {
                                self.environment.add_gloop(gloop.pile_column - 3 as u128);
                            }
                        }
                        None => (),
                    }
                }
            }
        }

        if self.machinery.engine.running {
            self.machinery.turn_handle();
        }

        self.machinery.move_machinery(&mut self.environment);

        for pile in self.environment.piles.values_mut() {
            let mut counter: u128 = 0;
            for gloop in pile {
                gloop.update_position(counter);
                counter += 1;
            }
        }

        // Move camera movement? put camera consts in that file
        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_mouse_scrolled_up(ctx) {
            if self.camera.scale.x <= MAX_CAMERA_SCALE {
                self.camera.scale += ZOOM_SPEED;
                //self.camera.position.x = mouse_pos.x;
                //self.camera.position.y = mouse_pos.y;
            }
        }
        if input::is_key_down(ctx, Key::LeftCtrl) && input::is_mouse_scrolled_down(ctx) {
            if self.camera.scale.x >= MIN_CAMERA_SCALE {
                self.camera.scale -= ZOOM_SPEED;
                //self.camera.position.x = mouse_pos.x;
                //self.camera.position.y = mouse_pos.y;
            }
        }

        if input::is_key_down(ctx, Key::W) {
            self.camera.position.y -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            // find a way to stop going under the floor
            self.camera.position.y += MOVEMENT_SPEED;
        }
        if input::is_key_down(ctx, Key::A) {
            self.camera.position.x -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::D) {
            self.camera.position.x += MOVEMENT_SPEED;
        }

        //for idler in &mut self.office.idlers {
        // idler.bounce();
        //}

        for hitter in &mut self.office.hitters {
            hitter.hit_engine(&mut self.machinery);
        }

        for mover in &mut self.office.movers {
            mover.move_gloop(&mut self.environment, &mut self.machinery.engine);
        }

        self.camera.update();

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::set_canvas(ctx, &self.background);

        // draw background
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

        // Drawing the labels and details
        let mut label = Text::new("Testing", self.assets.main_font.clone());
        self.button.draw(ctx, &self.assets);
        label.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(16., 16.))
                .color(Color::rgba(0., 75.3, 68.7, 1.)),
        );

        let mut gloops_label = Text::new(
            format!("Gloops: {}", self.environment.loose_gloop),
            self.assets.main_font.clone(),
        );
        gloops_label.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(16., 36.))
                .color(Color::BLUE),
        );

        let mut fuel_label = Text::new(
            format!("Fuel: {}", self.machinery.engine.fuel),
            self.assets.main_font.clone(),
        );

        fuel_label.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(16., 56.))
                .color(Color::BLUE),
        );

        graphics::set_transform_matrix(ctx, self.camera.as_matrix());
        self.machinery.draw(ctx, &self.assets);

        for pile in self.environment.piles.values_mut() {
            for gloop in pile {
                gloop.draw(ctx, &self.assets);
            }
        }

        for hitter in &self.office.hitters {
            self.assets.glooper_texture.draw(
                ctx,
                DrawParams::new()
                    .position(hitter.position)
                    .origin(Vec2::new(
                        self.assets.glooper_texture.width() as f32,
                        self.assets.glooper_texture.height() as f32,
                    ))
                    .scale(hitter.scale),
            )
        }

        for mover in &mut self.office.movers {
            mover.animation.advance(ctx);

            mover.animation.draw(
                ctx,
                DrawParams::new()
                    .position(mover.position)
                    .origin(Vec2::new(
                        self.assets.glooper_texture.width() as f32,
                        self.assets.glooper_texture.height() as f32,
                    ))
                    .scale(mover.scale),
            );

            if mover.gloop_collected {
                mover.gloop_held.unwrap().draw(ctx, &self.assets);
            }
        }

        for idler in &mut self.office.idlers {
            idler.animation.advance(ctx);
            idler.animation.draw(
                ctx,
                DrawParams::new()
                    .position(idler.position)
                    .origin(Vec2::new(
                        self.assets.glooper_texture.width() as f32,
                        self.assets.glooper_texture.height() as f32,
                    ))
                    .scale(idler.scale),
            )
        }

        self.assets.floor_texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(0., FLOOR_LEVEL))
                .scale(Vec2::new(150., 5.)),
        );

        graphics::reset_transform_matrix(ctx);

        graphics::reset_canvas(ctx);

        self.background.draw(ctx, DrawParams::new());
        self.canvas.draw(ctx, DrawParams::new());

        Ok(Transition::None)
    }
}
