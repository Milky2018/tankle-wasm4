use crate::enemy::*;
use crate::make_sprite;
use crate::sprite::*;
use crate::tank::*;

pub const ARMOR_SPRITE: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . . . . . . . . . . . .
    . . o o . . . o o . . .
    . o + + o . o + + o . .
    o + + + + o + + + + o .
    o + + + + + + + + + o .
    o + + + + + + + + + o .
    . o + + + + + + + o . .
    . . o + + + + + o . . .
    . . . o + + + o . . . .
    . . . . o + o . . . . .
    . . . . . o . . . . . .
    . . . . . . . . . . . .
};

pub const TRAJ_SPRITE: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . . . . . . . . . . . .
    o o o o o o o o . . . .
    o + x + + + + + o . . .
    o + x + + + + + + o . .
    o + x + + + + + + x o .
    o + x + + + + + + x . o
    o + x + + + + + + x o .
    o + x + + + + + + o . .
    o + x + + + + + o . . .
    o o o o o o o o . . . .
    . . . . . . . . . . . .
    . . . . . . . . . . . .
};

pub const CD_SPRITE: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . o o o o o o o o o . .
    . o . . . . . . . o . .
    . o . . . . . . . o . .
    . . o . . . . . o . . .
    . . . o x x x o . . . .
    . . . . o x o . . . . .
    . . . . o x o . . . . .
    . . . o x x x o . . . .
    . . o x x x x x o . . .
    . o x x x x x x x o . .
    . o x x x x x x x o . .
    . o o o o o o o o o . .
};

pub const SPEED_SPRITE: TwoBppSprite = make_sprite! {
    (TANK_WIDTH, TANK_WIDTH)
    . . . . . . . . . . . .
    . . . . . . . . . . . .
    . . o o o o o o o o o .
    . o + x + + x + + x + o
    . o x . x x . x x . x o
    . o o o o o o o o o + o
    o + x + + x + + x + o .
    o x . x x . x x . x o .
    o + x + + x + + x + o .
    . o o o o o o o o o . .
    . . . . . . . . . . . .
    . . . . . . . . . . . .
};

pub const ARMOR_BONUS: TankModel = TankModel {
    engine: 1,
    cooldown: 60,
    traj: 1,
    sprite: &ARMOR_SPRITE,
    controller: Controller::Stand,
    maxhp: 1,
    bonus: Bonus::Armor(1),
};

pub const TRAJ_BONUS: TankModel = TankModel {
    engine: 1,
    cooldown: 60,
    traj: 1,
    sprite: &TRAJ_SPRITE,
    controller: Controller::Stand,
    maxhp: 1,
    bonus: Bonus::Traj(2),
};

pub const CD_BONUS: TankModel = TankModel {
    engine: 1,
    cooldown: 60,
    traj: 1,
    sprite: &CD_SPRITE,
    controller: Controller::Stand,
    maxhp: 1,
    bonus: Bonus::Cd(3),
};

pub const SPEED_BONUS: TankModel = TankModel {
    engine: 1,
    cooldown: 60,
    traj: 1,
    sprite: &SPEED_SPRITE,
    controller: Controller::Stand,
    maxhp: 1,
    bonus: Bonus::Speed(1),
};

pub fn make_bonus() -> Vec<Tank> {
    vec![
        Tank::new(48, 60, Direction::Up, ARMOR_BONUS, Team::Bonus),
        Tank::new(48, 84, Direction::Up, TRAJ_BONUS, Team::Bonus),
        Tank::new(48, 108, Direction::Up, CD_BONUS, Team::Bonus),
        Tank::new(48, 132, Direction::Up, SPEED_BONUS, Team::Bonus),
    ]
}

#[derive(Clone)]
pub enum Bonus {
    None,
    Armor(i32),
    Traj(i32),
    Cd(i32),
    Speed(i32),
    Refill,
}

impl Bonus {
    pub fn up(&self, tank: &mut Tank) {
        match self {
            Bonus::Armor(n) => {
                tank.get_model().maxhp += n;
                tank.refill();
            }
            Bonus::Traj(n) => {
                tank.get_model().traj += n;
            }
            Bonus::Cd(n) => {
                tank.get_model().cooldown -= n;
            }
            Bonus::Speed(n) => {
                tank.get_model().engine += n;
            }
            Bonus::Refill => {
                tank.refill();
            }
            Bonus::None => {}
        }
    }
}