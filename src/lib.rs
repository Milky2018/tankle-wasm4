#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod sprite;
mod tank;
mod battle;
mod pos;
mod enemy;
mod ui;
mod collide;
mod scene;
mod level;
mod border;
mod bonus;

use scene::Scene;
use sprite::*;
use battle::*;
use lazy_static::lazy_static;
use ui::*;
use std::sync::Mutex;

struct Game {
    scenes: [Box<dyn Scene>; 4],
    scene_idx: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            scenes: [
                Box::new(Welcome), 
                Box::new(Battle::new()), 
                Box::new(Victory),
                Box::new(Defeat),
            ],
            scene_idx: 0,
        }
    }
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().expect("game_state");
    let idx = game.scene_idx;
    let new_idx = game.scenes[idx].update();
    game.scene_idx = new_idx;
}
