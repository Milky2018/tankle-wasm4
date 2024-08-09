use crate::bonus::Bonus;
use crate::border::*;
use crate::collide::collide;
use crate::collide::Collision;
use crate::make_sprite;
use crate::sprite::*;
use crate::tank::*;

pub const TANK1: TankModel = TankModel {
    engine: 2,
    cooldown: 20,
    traj: 4,
    sprite: &TANK_SPRITE1,
    controller: Controller::Player1,
    maxhp: 5,
    bonus: Bonus::None,
};

pub const TANK2: TankModel = TankModel {
    engine: 1,
    cooldown: 40,
    traj: 3,
    sprite: &TANK_SPRITE2,
    controller: Controller::Zombie,
    maxhp: 1,
    bonus: Bonus::None,
};

pub const TANK3: TankModel = TankModel {
    engine: 2,
    cooldown: 30,
    traj: 4,
    sprite: &TANK_SPRITE3,
    controller: Controller::Zombie,
    maxhp: 1,
    bonus: Bonus::None,
};

pub const TANK_SPRITE1: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . . . . . x . . . . . .
    . . . . . x . . . . . .
    . . . . . x . . . . . .
    o o . . + x + . . o o .
    + + + + + x + + + + + .
    o o + + + x + + + o o .
    + + + + x x x + + + + .
    o o + + x + x + + o o .
    + + + + x x x + + + + .
    o o + + + + + + + o o .
    + + + + + + + + + + + .
    o o + . . . . . + o o .
};

pub const TANK_SPRITE2: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . . . . . x . . . . . .
    o + . . . x . . . + o .
    + + . . . x . . . + + .
    o + + + x x x + + + o .
    + + + + + x + + + + + .
    o + + + x x x + + + o .
    + + + x + x + x + + + .
    o + + x + x + x + + o .
    + + + x + + + x + + + .
    o + + + x x x + + + o .
    + + + + + + + + + + + .
    o + . . . . . . . + o .
};

pub const TANK_SPRITE3: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . . . . x x x x . . . .
    o o + . . x x . . + o o
    + + + + . x x . + + + +
    o o + + + x x + + + o o
    + + + + + x x + + + + +
    o o + + + x x + + + o o
    + + + x x x x x x + + +
    o o + x + x + + x + o o
    + + + x + + x + x + + +
    o o + x x x x x x + o o
    + + + + + + + + + + + +
    o o + . . . . . . + o o
};

pub fn spawn_enemy_pos(occupied: &Vec<Collision>) -> Option<(i32, i32, Direction)> {
    let pos1 = (BORDER_LEFT, BORDER_TOP, Direction::Down);
    let pos2 = (
        BORDER_RIGHT - TANK_WIDTH as i32,
        BORDER_TOP,
        Direction::Down,
    );
    let pos3 = (
        BORDER_RIGHT - TANK_WIDTH as i32,
        BORDER_BOTTOM - TANK_WIDTH as i32,
        Direction::Up,
    );
    let pos4 = (
        BORDER_LEFT,
        BORDER_BOTTOM - TANK_WIDTH as i32,
        Direction::Up,
    );
    for (x, y, dir) in vec![pos1, pos2, pos3, pos4] {
        let col = Collision::square(x, y, TANK_WIDTH);
        if occupied.iter().all(|collision| !collide(&col, collision)) {
            return Option::Some((x, y, dir));
        }
    }
    Option::None
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Controller {
    Player1,
    StandShoot,
    Stand,
    Zombie,
}
