pub const WELCOME_SCENE: usize = 0;
pub const BATTLE_SCENE: usize = 1;
pub const SUCCESS_SCENE: usize = 2;
pub const DEFEAT_SCENE: usize = 3;

pub trait Scene: Send {
    fn update(&mut self) -> usize;
}