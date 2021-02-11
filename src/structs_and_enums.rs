pub use crate::game_data::*;
pub use ggez::audio::{self, *};
pub use ggez::event::{self, EventHandler};
pub use ggez::graphics::{
    self, spritebatch::SpriteBatch, Color, DrawMode, DrawParam, Drawable, Image, Mesh, MeshBuilder,
    Rect, Text,
};
pub use ggez::input::keyboard::*;
pub use ggez::nalgebra::Point2;
pub use ggez::{conf, filesystem, input::mouse, Context, ContextBuilder, GameResult};
pub use rand;
use std::io::{Read, Write};
pub struct Menu {
    pub active: bool,
    pub start_button: Button,
}
pub struct Button {
    pub bounds: Rect,
    pub mesh: Mesh,
    pub text: Text,
}

impl Menu {
    pub fn new(ctx: &mut Context) -> Menu {
        let sb_rect = Rect::new(810.0, 500.0, 300.0, 100.0);
        let sb = Mesh::new_rectangle(ctx, DrawMode::fill(), sb_rect, Color::from_rgb(70, 70, 70))
            .unwrap();
        let mut text = Text::new("Start");
        text.set_font(
            graphics::Font::default(),
            graphics::Scale { x: 40.0, y: 40.0 },
        );
        let button = Button {
            bounds: sb_rect,
            mesh: sb,
            text: text,
        };

        let m = Menu {
            active: true,
            start_button: button,
        };
        m
    }
    pub fn get_start_button(&self) -> &Button {
        &self.start_button
    }
}
pub struct Game {
    // lpoints: Vec<Point2<f32>>,
    // rpoints: Vec<Point2<f32>>,
    // gap_width: f32,
    pub menu: Menu,
    pub game_data: GameData,
    pub exit: bool,
    pub hs: usize,
    pub music: audio::Source,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        let mut s = audio::Source::new(ctx, "/musikskit.wav").expect("audio error");
        s.set_repeat(true);
        s.set_volume(0.3);
        s.play().expect("audio playing error");
        Game {
            // lpoints: create_init_points(true),
            // rpoints: create_init_points(false),
            // gap_width: 300.0,
            game_data: GameData::default(ctx),
            menu: Menu::new(ctx),
            hs: 0,
            exit: false,
            music: s,
        }
    }
    pub fn get_highscore(ctx: &mut Context) -> usize {
        match filesystem::open(ctx, "/scores.txt") {
            Ok(mut x) => {
                let mut buf = String::new();
                x.read_to_string(&mut buf).expect("error reading file");
                let mut current_largest: usize = 0;
                for st in buf.split_whitespace() {
                    let c = st.parse::<usize>().expect("parsing error");
                    if c > current_largest {
                        current_largest = c;
                    }
                }
                return current_largest;
            }
            Err(_) => {
                return 0;
            }
        }
    }
    pub fn save_score(ctx: &mut Context, score: usize) {
        let options = filesystem::OpenOptions::new().append(true);
        let file = filesystem::open_options(ctx, "/scores.txt", options);
       
        let b = format!("{} ", score);
        let b = b.as_str().as_bytes();
        match file {
            Ok(mut f) => {
                f.write(b).expect("error writing to file");
            }
            Err(_) => {
                println!("couldnt open file. creating one");
                let mut t = filesystem::create(ctx, "/scores.txt").expect("error creating file");
                t.write_all(b).expect("error writing to file2");
            }
        }
    }

    // pub fn modify_points(&mut self) {
    //     self.lpoints.remove(0);
    //     self.rpoints.remove(0);
    //     for n in &mut self.lpoints {
    //         n.y += 10.0;
    //     }
    //     for n in &mut self.rpoints {
    //         n.y += 10.0;
    //     }
    //     let prev_x = self
    //         .lpoints
    //         .as_mut_slice()
    //         .last()
    //         .expect("mod_p err")
    //         .as_ref()
    //         .x;
    //     let mut x = (rand::random::<i64>() % 20) as f32 + prev_x;
    //     if x < 0.0 {
    //         x = 0.0;
    //     }
    //     if x > 1920.0 - self.gap_width {
    //         x = 1920.0 - self.gap_width;
    //     }
    //     self.lpoints.push(Point2::new(x as f32, 0.0));
    //     self.rpoints
    //         .push(Point2::new((x + self.gap_width) as f32, 0.0));
    // }
}
