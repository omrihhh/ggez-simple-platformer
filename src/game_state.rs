use crate::player::Player;
use crate::tiles;

pub struct GameState {
    pub player: Player,
    pub platforms: Vec<(i16,i16,i16)>
} 

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(0.0, 24.0),
            platforms: tiles::gen_platforms()
        }
    }
}