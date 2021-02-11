pub use crate::{game_data::*, power_ups::*};
pub use ggez::graphics::{
    self, Color, DrawMode, DrawParam, Drawable, Mesh, MeshBuilder, Rect, Text,
};
pub use ggez::nalgebra::{Point2, Vector2};
#[derive(Copy, Clone)]
pub struct GameObject {
    pub radius: f32,
    pub draw_param: DrawParam,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub obj_type: ObjType,
    pub rot_coef: f32,
}
#[derive(Copy, Clone)]
pub enum ObjType {
    Asteroid,
    Player,
    PowerUp(PowerUpType),
}

impl GameObject {
    const BASE_SPEED: f32 = 7.0;
    const BASE_RADIUS: f32 = 15.0;
    pub fn new(t: ObjType, data: &mut GameData) -> GameObject {
        match t {
            ObjType::Asteroid => GameObject::new_asteroid(data),
            ObjType::Player => GameObject::new_player(),
            ObjType::PowerUp(_) => GameObject::new_pwr_up(),
        }
    }
    pub fn get_draw_param(&self) -> DrawParam {
        self.draw_param
    }

    pub fn get_pw_type(&self) -> Option<PowerUpType> {
        match self.obj_type {
            ObjType::PowerUp(t) => return Some(t),
            _ => return None,
        }
    }

    pub fn new_asteroid(data: &mut GameData) -> GameObject {
        let max_increased_speed = data.score as f32 / (1200.0);
        let min_speed = GameObject::BASE_SPEED + max_increased_speed / 4.0;
        let speed = (rand::random::<u32>() % f32::floor(max_increased_speed + 1.0) as u32) as f32
            + min_speed;
        let mut max_increased_radius = 1.0 + f32::floor(data.score as f32 / (30.0));
        if max_increased_radius >= 70.0 {
            max_increased_radius = 70.0;
        }
        // println!("speed: {}", speed);
        let radius = f32::floor(
            GameObject::BASE_RADIUS
                + (rand::random::<u32>() % (max_increased_radius as u32)) as f32,
        );
        let x = rand::random::<u32>() as f32 % (1920.0 - 2.0 * radius) + radius;
        let mut y = -radius;
        if data.score == 0 {
            y = -1.0 * (rand::random::<u32>() % 1080) as f32 - radius;
        }
        let rot = std::f32::consts::PI;
        let scale = radius / 100.0;
        let rot_coef = ((rand::random::<u8>() % 4) + 1) as f32 / 40.0 - 2.0 / 40.0;
        let dp = DrawParam::new()
            .src(Rect::new(0.0, 0.0, 1.0 / 7.0, 1.0))
            .dest(Point2::new(0.0, 0.0))
            .offset(Point2::new(0.5, 0.5))
            .rotation(rot)
            .scale(Vector2::new(scale, scale));
        let asteroid = GameObject {
            radius: radius,
            x: x,
            y: y,
            speed: speed,
            draw_param: dp,
            obj_type: ObjType::Asteroid,
            rot_coef: rot_coef,
        };
        // let mut tries = 0;
        // while tries < 50 {
        //     if data.get_asteroid_collision(&mut asteroid) {
        //         asteroid.x = rand::random::<u32>() as f32 % (1920.0 - 2.0 * radius) + radius;
        //         tries += 1;
        //         continue;
        //     } else {
        //         break;
        //     }
        // }
        // asteroid.draw_param = dp.dest(Point2::new(asteroid.x, y));

        asteroid
    }

    pub fn new_player() -> GameObject {
        let x = 960.0;
        let y = 850.0;
        let dp = DrawParam::new()
            .src(Rect::new(1.0 / 7.0, 0.0, 1.0 / 7.0, 1.0))
            .dest(Point2::new(x, y))
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(0.33, 0.33));
        GameObject {
            radius: 22.0,
            draw_param: dp,
            x: x,
            y: y,
            speed: 10.0,
            rot_coef: 0.0,
            obj_type: ObjType::Player,
        }
    }

    pub fn new_pwr_up() -> GameObject {
        let radius = 25.0;
        let x = rand::random::<u32>() as f32 % (1920.0 - 2.0 * radius) + radius;
        let y = -radius;
        let mut pw = PowerUpType::TimeSlow;
        let mut src_k: f32 = 0.0;
        let a = rand::random::<usize>() % 4;
        for (n, p) in [
            PowerUpType::Invincible,
            PowerUpType::Small,
            PowerUpType::ShrinkAsteroids,
            PowerUpType::TimeSlow,
        ]
        .iter()
        .enumerate()
        {
            if n == a {
                pw = *p;
                src_k = (n + 3) as f32 / 7.0 + 1.0 / 600.0;
            }
        }
        let dp = DrawParam::new()
            .src(Rect::new(src_k, 0.0, 1.0 / 7.0, 1.0))
            .dest(Point2::new(x, y))
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(0.25, 0.25));
        GameObject {
            radius: radius,
            x: x,
            y: y,
            speed: 3.0,
            rot_coef: 0.0,
            obj_type: ObjType::PowerUp(pw),
            draw_param: dp,
        }
    }
}
