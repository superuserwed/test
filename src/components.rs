#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub struct Renderable {
    pub path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {}

pub struct BoxSpot {}

pub struct Movable;

pub struct Immovable;
#[derive(Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}