use crate::game_object::*;
use crate::power_ups::*;
use crate::structs_and_enums::*;
pub struct GameData {
    start_time: std::time::SystemTime,
    asteroid_vec: Vec<GameObject>,
    pub pup: PowerUpHandler,
    sprite_batch: SpriteBatch,
    asteroid_count: u32,
    player: GameObject,
    pub score: usize,
    pub background: BackgroundImage,
}

pub struct BackgroundImage {
    pub img: Image,
    pub y: f32,
}

impl BackgroundImage {
    fn new(ctx: &mut Context) -> BackgroundImage {
        BackgroundImage {
            img: Image::new(ctx, "/space.png").expect("image error"),
            y: 0.0,
        }
    }
}

impl GameData {
    pub fn default(ctx: &mut Context) -> GameData {
        let img = Image::new(ctx, "/sprite.png").expect("image loading error");
        let sprite_batch = SpriteBatch::new(img);
        GameData {
            start_time: std::time::SystemTime::now(),
            asteroid_vec: Vec::<GameObject>::new(),
            pup: PowerUpHandler::default(),
            sprite_batch: sprite_batch,
            asteroid_count: 14,
            player: GameObject::new_player(),
            score: 0,
            background: BackgroundImage::new(ctx),
        }
    }

    pub fn get_score(&mut self) -> &mut usize {
        &mut self.score
    }

    pub fn check_if_get_pup(&mut self) {
        let mut remove: Vec<usize> = Vec::new();
        let player = self.get_player().clone();
        for (n, pup) in self.pup.pups_on_screen.iter().enumerate() {
            let center_distance =
                f32::sqrt(f32::powi(player.x - pup.x, 2) + f32::powi(player.y - pup.y, 2));
            if center_distance <= (player.radius + pup.radius) && self.pup.owned_pups.len() < 3 {
                remove.push(n);
                self.pup.owned_pups.push(*pup);
            }
        }
        for n in remove {
            self.pup.pups_on_screen.remove(n);
        }
    }

    pub fn get_asteroid_collision(&mut self, obj: &mut GameObject) -> bool {
        let mut remove: Vec<usize> = Vec::new();
        let mut coll = false;
        for (n, asteroid) in self.asteroid_vec.iter().enumerate() {
            let center_distance =
                f32::sqrt(f32::powi(asteroid.x - obj.x, 2) + f32::powi(asteroid.y - obj.y, 2));
            if center_distance <= (asteroid.radius + obj.radius) {
                match obj.obj_type {
                    ObjType::Player => match self.pup.active {
                        Some(PowerUpType::Invincible) => {
                            remove.push(n);
                            break;
                        }
                        _ => {
                            println!("collision, player at {}:{}, ast at {}:{}. ast_radius: {}, ast_speed: {}, ast_rot:{}, ast_rot_coef: {}, ast_dp_x: {}, ast_dp_y:{}", obj.x,obj.y, asteroid.x,asteroid.y, asteroid.radius, asteroid.speed, asteroid.draw_param.rotation, asteroid.rot_coef, asteroid.draw_param.dest.x, asteroid.draw_param.dest.y);
                            remove.push(n);
                            coll = true;
                        }
                    },
                    ObjType::PowerUp(_) => {}
                    ObjType::Asteroid => return true,
                }
            }
        }
        if coll {
            self.asteroid_vec.remove(*remove.get(0).unwrap());
            return true;
        }
        for n in remove {
            self.asteroid_vec.remove(n);
        }
        false
    }

    pub fn get_player(&mut self) -> &mut GameObject {
        &mut self.player
    }

    pub fn move_player(&mut self, ctx: &mut Context, direction: KeyCode) {
        let mut speed = self.player.speed;

        if is_mod_active(ctx, KeyMods::SHIFT) {
            speed *= 2.0;
        }

        match direction {
            KeyCode::Up => {
                if self.player.y > 0.0 + self.player.radius {
                    self.player.y -= speed;
                } else {
                    self.player.y = 0.0 + self.player.radius;
                }
            }
            KeyCode::Down => {
                if self.player.y < 1080.0 - self.player.radius {
                    self.player.y += speed;
                } else {
                    self.player.y = 1080.0 - self.player.radius;
                }
            }
            KeyCode::Left => {
                if self.player.x > 0.0 + self.player.radius {
                    self.player.x -= speed;
                } else {
                    self.player.x = 0.0 + self.player.radius;
                }
            }
            KeyCode::Right => {
                if self.player.x < 1920.0 - self.player.radius {
                    self.player.x += speed;
                } else {
                    self.player.x = 1920.0 - self.player.radius;
                }
            }
            _ => println!("Invalid keycode"),
        }
        self.player.draw_param = self
            .player
            .draw_param
            .clone()
            .dest(Point2::new(self.player.x, self.player.y));
    }

    pub fn get_time_elapsed(&self) -> f32 {
        self.start_time.elapsed().expect("time error").as_secs_f32()
    }
    pub fn get_asteroids(&mut self) -> &mut Vec<GameObject> {
        &mut self.asteroid_vec
    }
    pub fn get_sprite_batch(&mut self) -> &mut SpriteBatch {
        &mut self.sprite_batch
    }
    pub fn get_asteroid_count(&self) -> &u32 {
        &self.asteroid_count
    }
    pub fn move_objects(&mut self) {
        let mut remove_ast: usize = 0;
        let mut remove_pup = false;
        let mut combined_vec: Vec<&mut GameObject> = Vec::new();
        for x in &mut self.asteroid_vec {
            combined_vec.push(x);
        }
        for x in &mut self.pup.pups_on_screen {
            combined_vec.push(x);
        }
        for (n, obj) in combined_vec.iter_mut().enumerate() {
            let new_y: f32;
            if let Some(PowerUpType::TimeSlow) = self.pup.active {
                new_y = obj.y + obj.speed / 2.0;
            } else {
                new_y = obj.y + obj.speed;
            }

            if new_y > 1080.0 + obj.radius * 2.0 {
                match obj.obj_type {
                    ObjType::Asteroid => {
                        remove_ast = n + 1;
                    }
                    ObjType::Player => {}
                    ObjType::PowerUp(_) => {
                        remove_pup = true;
                    }
                }
                continue;
            }
            let new_rot = obj.get_draw_param().rotation + obj.rot_coef;
            obj.draw_param = obj
                .draw_param
                .dest(Point2::new(obj.x, new_y))
                .rotation(new_rot);
            obj.y = new_y;
        }
        if remove_pup {
            &mut self.pup.pups_on_screen.remove(0);
        }
        // let mut add: u8 = 0;
        if remove_ast > 0 {
            &mut self.asteroid_vec.remove(remove_ast - 1);
        }

        // add +=1;

        // if add > 0{
        //     for _ in 0..add{
        //         let a = GameObject::new(ObjType::Asteroid, self);
        //         &mut self.asteroid_vec.push(a);
        //     }
        // }
    }
}
