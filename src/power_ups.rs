pub use crate::game_object::*;
pub struct PowerUpHandler {
    pub owned_pups: Vec<GameObject>,
    pub pups_on_screen: Vec<GameObject>,
    pub active: Option<PowerUpType>,
    pub active_dur: i32,
}

impl PowerUpHandler {
    pub fn default() -> PowerUpHandler {
        PowerUpHandler {
            owned_pups: Vec::<GameObject>::new(),
            pups_on_screen: Vec::<GameObject>::new(),
            active: None,
            active_dur: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum PowerUpType {
    TimeSlow,
    Invincible,
    Small,
    ShrinkAsteroids,
}
