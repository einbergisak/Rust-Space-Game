pub use ggez::event::{self, EventHandler};
pub use ggez::graphics::{self, Color, DrawMode, Drawable, Mesh, MeshBuilder, Rect, Text};
pub use ggez::nalgebra::{Point2, Vector2};
pub use ggez::{
    conf, filesystem,
    input::keyboard::{self, *},
    input::mouse,
    Context, ContextBuilder, GameResult,
};
pub use rand;
pub mod game_data;
pub mod game_object;
pub mod power_ups;
pub mod structs_and_enums;
pub use game_data::*;
pub use power_ups::*;
pub use structs_and_enums::*;
fn main() {
    let mut path;
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        loop {
            path = std::path::PathBuf::from(manifest_dir.clone());
            path.push("src");
            path.push("resources");

            let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
                .window_mode(
                    conf::WindowMode::default()
                        .dimensions(1920.0, 1080.0)
                        .fullscreen_type(ggez::conf::FullscreenType::Desktop)
                        .maximized(true)
                        .resizable(false),
                )
                .add_resource_path(path)
                .build()
                .expect("contextbuilder fail");
            mouse::set_cursor_grabbed(&mut ctx, true).expect("cursor grab failed");
            graphics::set_drawable_size(&mut ctx, 1920.0, 1080.0).expect("window drawable fail");
            graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, 1920.0, 1080.0))
                .expect("screen coord fail");

            let mut my_game = Game::new(&mut ctx);
            // Run!
            match event::run(&mut ctx, &mut event_loop, &mut my_game) {
                Ok(_) => {
                    if my_game.exit {
                        println!("Exited cleanly.");
                        break;
                    } else {
                        continue;
                    }
                }
                Err(e) => println!("Error occured: {}", e),
            }
        }
    } else {
        println!("Error loading file.");
    }
}

// fn create_init_points(left_side: bool) -> Vec<Point2<f32>> {
//     //initial points
//     let mut pvec: Vec<Point2<f32>> = Vec::new();
//     let k;
//     if left_side {
//         k = -1.0
//     } else {
//         k = 1.0;
//     }
//     for y in 1..=108 {
//         // = 1080/10. lodrätt avstånd mellan varje point
//         pvec.push(Point2::<f32>::new(
//             960.0 + 150.0 * k,
//             (1080 - y * 10) as f32,
//         ));
//     }
//     pvec
// }
const FPS: u32 = 60;
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, FPS) {
            if self.menu.active {
            } else {
                *self.game_data.get_score() += 1;

                if self.game_data.pup.active_dur > 0 {
                    self.game_data.pup.active_dur -= 1;
                } else {
                    self.game_data.pup.active_dur = 0;
                }

                if self.game_data.pup.active_dur == 0 {
                    if let Some(t) = self.game_data.pup.active {
                        match t {
                            PowerUpType::TimeSlow => {}
                            PowerUpType::Invincible => {
                                self.game_data.get_player().draw_param.src =
                                    Rect::new(1.0 / 7.0, 0.0, 1.0 / 7.0, 1.0);
                            }
                            PowerUpType::Small => {
                                let s = self.game_data.get_player().get_draw_param().scale;
                                self.game_data.get_player().draw_param.scale =
                                    Vector2::new(s.x * 2.0, s.y * 2.0).into();
                                self.game_data.get_player().radius *= 2.0;
                            }
                            PowerUpType::ShrinkAsteroids => {}
                        }
                    }
                    self.game_data.pup.active = None;
                }

                // self.modify_points();
                // ggez::timer::sleep(core::time::Duration::new(
                //     0,
                //     f32::floor(5000000000.0 / self.game_speed) as u32,
                // ));
                // let t = self
                //     .start_time
                //     .duration_since(self.start_time)
                //     .expect("time error")
                //     .as_secs_f32();

                // //Fixa så att gap width och speed ändras linjärt med vanlig tid
                // //För det så behöver du lägga till "default speed"
                // //och "default gap_width", sedan ändra gap width till default_gap_width - k*t
                // self.game_speed += 0.01;
                // if self.gap_width > 100.0 {
                //     self.gap_width -= 1.0;
                // }
                // if *self.game_data.get_score() % 10 == 0 {
                //     let a = GameObject::new(ObjType::Asteroid, &mut self.game_data);

                //     self.game_data.get_asteroids().push(a);
                // }

                if *self.game_data.get_score() % 1000 == 0 {
                    let b = GameObject::new_pwr_up();
                    self.game_data.pup.pups_on_screen.push(b);
                }

                if *self.game_data.get_score() % 8 == 0 {
                    let a = GameObject::new(ObjType::Asteroid, &mut self.game_data);
                    &mut self.game_data.get_asteroids().push(a);
                }

                self.game_data.move_objects();

                self.game_data.check_if_get_pup();

                for direction in [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right].iter()
                {
                    if keyboard::is_key_pressed(ctx, *direction) {
                        self.game_data.move_player(ctx, *direction);
                    }
                }
                let mut p = self.game_data.get_player().clone();

                if self.game_data.get_asteroid_collision(&mut p) {
                    let score = *self.game_data.get_score();
                    let rect = Rect::new_i32(560, 290, 800, 300);

                    let mut text = Text::new(format!("Final score: {}", score));
                    text.set_font(
                        graphics::Font::default(),
                        graphics::Scale { x: 70.0, y: 70.0 },
                    );
                    let mesh = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        rect,
                        Color::from_rgb(100, 100, 100),
                    )?;
                    graphics::draw(ctx, &mesh, (Point2::<f32>::new(0.0, 0.0),))?;
                    graphics::draw(ctx, &text, (Point2::<f32>::new(700.0, 400.0),))?;
                    graphics::present(ctx).ok();

                    Game::save_score(ctx, score);

                    ggez::timer::sleep(std::time::Duration::from_secs(4));
                    self.music.stop();
                    ggez::event::quit(ctx);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        if self.menu.active {
            let mut hs_text = Text::new(format!("Highscore: {}", Game::get_highscore(ctx)));
            hs_text.set_font(
                graphics::Font::default(),
                graphics::Scale { x: 40.0, y: 40.0 },
            );
            hs_text.draw(
                ctx,
                DrawParam::default().dest(Point2::<f32>::new(30.0, 30.0)),
            )?;

            let mut sb = &mut self.menu.start_button;

            if sb.bounds.contains(mouse::position(ctx)) {
                sb.mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    sb.bounds,
                    Color::from_rgb(150, 150, 150),
                )
                .unwrap()
            } else {
                sb.mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    sb.bounds,
                    Color::from_rgb(70, 70, 70),
                )
                .unwrap()
            }
            graphics::draw(ctx, &sb.mesh, (Point2::<f32>::new(0.0, 0.0),))?;
            graphics::draw(ctx, &sb.text, (Point2::new(860.0, 530.0),))?;
        } else {
            // let lp = self.lpoints.clone().into_boxed_slice();
            // let rp = self.rpoints.clone().into_boxed_slice();
            // let left_line = graphics::Mesh::new_polyline(
            //     ctx,
            //     graphics::DrawMode::stroke(3.0),
            //     &lp,
            //     graphics::WHITE,
            // )?;
            // let right_line = graphics::Mesh::new_polyline(
            //     ctx,
            //     graphics::DrawMode::stroke(3.0),
            //     &rp,
            //     graphics::WHITE,
            // )?;

            // graphics::draw(ctx, &left_line, (Point2::<f32>::new(0.0, 0.0),))?;
            // graphics::draw(ctx, &right_line, (Point2::<f32>::new(0.0, 0.0),))?;
            let bg = &self.game_data.background;
            let dp_b = DrawParam::default().dest(Point2::new(0.0, bg.y));
            bg.img.draw(ctx, dp_b)?;

            let mut pup_time =
                Text::new(f32::ceil(self.game_data.pup.active_dur as f32 / 60.0).to_string());
            pup_time.set_font(
                graphics::Font::default(),
                graphics::Scale { x: 700.0, y: 700.0 },
            );

            if self.game_data.pup.active.is_some() {
                graphics::draw(
                    ctx,
                    &pup_time,
                    DrawParam::new()
                        .color(Color::from_rgb(70, 70, 70))
                        .dest(Point2::<f32>::new(720.0, 190.0)),
                )?;
            }

            let mut dp_vec: Vec<DrawParam> = Vec::new();

            let pdp = self.game_data.get_player().get_draw_param();
            self.game_data.get_sprite_batch().add(pdp);

            let mut score = Text::new(self.game_data.get_score().to_string());
            score.set_font(
                graphics::Font::default(),
                graphics::Scale { x: 30.0, y: 30.0 },
            );

            let m = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new_i32(0, 0, 150, 50),
                Color::from_rgb(120, 120, 120),
            )?;
            graphics::draw(ctx, &m, (Point2::<f32>::new(0.0, 1030.0),))?;

            for (n, x) in ["Q", "W", "E"].iter().enumerate() {
                // let r = Rect::new(0.0, 0.0, 48.0, 48.0);
                // let r = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), r, graphics::WHITE)?;
                // graphics::draw(ctx, &r, (Point2::<f32>::new(n as f32 * 50.0, 1030.0),))?;

                let mut t = Text::new(*x);
                t.set_font(
                    graphics::Font::default(),
                    graphics::Scale { x: 30.0, y: 30.0 },
                );

                graphics::draw(
                    ctx,
                    &t,
                    (Point2::<f32>::new(17.0 + (n * 50) as f32, 1000.0),),
                )?;
            }

            for x in &mut self.game_data.pup.pups_on_screen {
                dp_vec.push(x.draw_param);
            }
            for x in self.game_data.get_asteroids() {
                let dp = x.get_draw_param().dest(Point2::new(x.x, x.y));
                dp_vec.push(dp);
            }

            for (n, obj) in self.game_data.pup.owned_pups.iter().enumerate() {
                let dp = obj
                    .get_draw_param()
                    .offset(Point2::new(0.0, 0.0))
                    .dest(Point2::<f32>::new((n * 50) as f32, 1030.0));
                dp_vec.push(dp);
            }

            for x in dp_vec {
                self.game_data.get_sprite_batch().add(x);
            }

            graphics::draw(
                ctx,
                self.game_data.get_sprite_batch(),
                (Point2::<f32>::new(0.0, 0.0),),
            )?;

            graphics::draw(ctx, &score, (Point2::<f32>::new(0.0, 0.0),))?;
        }

        self.game_data.get_sprite_batch().clear();

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // while ggez::timer::check_update_time(ctx, FPS){
        //     if keycode == KeyCode::Up{

        //     }
        //     if keycode == KeyCode::Down{

        //     }
        //     if keycode == KeyCode::Left{

        //     }
        //     if keycode == KeyCode::Right{

        //     }
        // }

        if keycode == KeyCode::Escape {
            self.exit = true;
            ggez::event::quit(ctx);
        }

        if keycode == KeyCode::Return && self.menu.active {
            self.menu.active = false;
            for _ in 0..*self.game_data.get_asteroid_count() {
                let a = GameObject::new(ObjType::Asteroid, &mut self.game_data);
                self.game_data.get_asteroids().push(a);
            }
        }

        if !self.menu.active && self.game_data.pup.active.is_none() {
            let gd = &mut self.game_data;
            let mut pup_kind: PowerUpType = PowerUpType::TimeSlow;
            for (n, x) in [KeyCode::Q, KeyCode::W, KeyCode::E].iter().enumerate() {
                if &keycode == x && gd.pup.owned_pups.len() > n {
                    let p = gd.pup.owned_pups.get(n).expect("pup activation error");
                    match p.obj_type {
                        ObjType::Asteroid => {}
                        ObjType::Player => {}
                        ObjType::PowerUp(t) => {
                            gd.pup.owned_pups.remove(n);
                            pup_kind = t;
                        }
                    }
                    match pup_kind {
                        PowerUpType::TimeSlow => {}
                        PowerUpType::Invincible => {
                            gd.get_player().draw_param.src =
                                Rect::new(2.0 / 7.0, 0.0, 1.0 / 7.0, 1.0);
                        }
                        PowerUpType::Small => {
                            let s = gd.get_player().draw_param.scale;
                            gd.get_player().draw_param.scale =
                                Vector2::new(s.x / 2.0, s.y / 2.0).into();
                            gd.get_player().radius /= 2.0;
                        }
                        PowerUpType::ShrinkAsteroids => {
                            for x in self.game_data.get_asteroids() {
                                x.radius /= 2.0;
                                let s = x.get_draw_param().scale;
                                x.draw_param.scale = x
                                    .draw_param
                                    .clone()
                                    .scale(Vector2::new(s.x / 2.0, s.y / 2.0))
                                    .scale;
                            }
                            return;
                        }
                    }

                    gd.pup.active = Some(pup_kind);
                    gd.pup.active_dur = 60 * 7;
                }
            }
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == mouse::MouseButton::Left && self.menu.active {
            let b_min_x = self.menu.start_button.bounds.x;
            let b_max_x = self.menu.start_button.bounds.w + b_min_x;
            let b_min_y = self.menu.start_button.bounds.y;
            let b_max_y = self.menu.start_button.bounds.h + b_min_y;
            if x > b_min_x && x < b_max_x && y > b_min_y && y < b_max_y {
                self.menu.active = false;
                for _ in 0..*self.game_data.get_asteroid_count() {
                    let a = GameObject::new(ObjType::Asteroid, &mut self.game_data);
                    self.game_data.get_asteroids().push(a);
                }
            }
        }
    }
}
