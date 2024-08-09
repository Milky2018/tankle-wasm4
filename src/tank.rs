use oorandom::Rand32;

use crate::bonus::Bonus;
use crate::border::*;
use crate::collide::collide;
use crate::collide::Collision;
use crate::enemy::Controller;
use crate::pos::*;
use crate::sprite::*;
use crate::wasm4::*;

#[derive(Clone)]
pub struct TankModel {
    pub engine: i32,
    pub cooldown: i32,
    pub traj: i32,
    pub sprite: &'static TwoBppSprite,
    pub controller: Controller,
    pub maxhp: i32,
    pub bonus: Bonus,
}

pub struct Tank {
    live: bool,
    x: i32,
    y: i32,
    direction: Direction,
    speed: i32,
    canon_heat: i32,
    model: TankModel,
    team: Team,
    life: i32,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Team {
    Players,
    Enemies,
    Bonus,
}

impl Tank {
    pub fn new(x: i32, y: i32, direction: Direction, model: TankModel, team: Team) -> Tank {
        Tank {
            live: true,
            x,
            y,
            direction,
            speed: 0,
            canon_heat: 0,
            life: model.maxhp,
            model,
            team,
        }
    }
    
    pub fn refill(&mut self) {
        self.life = self.model.maxhp;
    }

    pub fn get_team(&self) -> Team {
        self.team.clone()
    }

    pub fn is_live(&self) -> bool {
        self.live
    }

    pub fn get_model(&mut self) -> &mut TankModel {
        &mut self.model
    }

    pub fn get_life(&self) -> i32 {
        self.life
    }

    pub fn update(&mut self, collisions: &Vec<Collision>) {
        if self.speed != 0 {
            let (new_x, new_y) = translate_point(self.x, self.y, self.direction, self.speed);
            let others: Vec<Collision> = collisions
                .clone()
                .into_iter()
                .filter(|col| *col != self.collision())
                .collect();

            if Tank::check_collide(new_x, new_y, &others) {
                self.speed -= 1;
                self.update(&others);
            } else {
                self.y = new_y;
                self.x = new_x;
            }
        }
        if self.canon_heat > 0 {
            self.canon_heat -= 1;
        }
    }

    pub fn collision(&self) -> Collision {
        Collision::square(self.x, self.y, TANK_WIDTH)
    }

    fn check_collide(x: i32, y: i32, others: &Vec<Collision>) -> bool {
        let tank_ent = Collision::square(x, y, TANK_WIDTH);
        for enemy in others.iter() {
            if collide(&tank_ent, &enemy) {
                return true;
            }
        }
        if x >= BORDER_LEFT
            && x <= BORDER_RIGHT - TANK_WIDTH as i32
            && y >= BORDER_TOP
            && y <= BORDER_BOTTOM - TANK_WIDTH as i32
        {
            return false;
        }
        true
    }

    pub fn draw(&self) {
        if self.live {
            self.model.sprite.draw(self.x, self.y, self.direction);
        }
    }

    fn turn_and_move(&mut self, dir: Direction) {
        self.direction = dir;
        self.speed = self.model.engine;
    }

    fn shoot(&mut self) -> Option<Bullet> {
        if self.canon_heat == 0 {
            let center_x = self.x + TANK_WIDTH as i32 / 2 - BULLET_WIDTH as i32 / 2;
            let center_y = self.y + TANK_WIDTH as i32 / 2 - BULLET_WIDTH as i32 / 2;

            let (x, y) = translate_point(
                center_x,
                center_y,
                self.direction,
                TANK_WIDTH as i32 / 2 + BULLET_WIDTH as i32 / 2,
            );

            self.canon_heat += self.model.cooldown;

            Option::Some(Bullet::new(
                x,
                y,
                self.direction,
                self.model.traj,
                self.team.clone(),
            ))
        } else {
            Option::None
        }
    }

    pub fn control(&mut self, rng: &mut Rand32) -> Option<Bullet> {
        match self.model.controller {
            Controller::Player1 => {
                let gamepad = unsafe { *GAMEPAD1 };
                let dir = if gamepad & BUTTON_LEFT != 0 {
                    Option::Some(Direction::Left)
                } else if gamepad & BUTTON_RIGHT != 0 {
                    Option::Some(Direction::Right)
                } else if gamepad & BUTTON_DOWN != 0 {
                    Option::Some(Direction::Down)
                } else if gamepad & BUTTON_UP != 0 {
                    Option::Some(Direction::Up)
                } else {
                    Option::None
                };
                if let Some(dir) = dir {
                    self.turn_and_move(dir);
                } else {
                    self.speed = 0;
                }
                if gamepad & BUTTON_1 != 0 {
                    return self.shoot();
                } else {
                    return Option::None;
                }
            }
            Controller::StandShoot => {
                return self.shoot();
            }
            Controller::Stand => {
                return None;
            }
            Controller::Zombie => {
                let rate = rng.rand_range(0..1000);

                if rate > 980 {
                    let dir = if rate > 995 {
                        Direction::Up
                    } else if rate > 990 {
                        Direction::Down
                    } else if rate > 985 {
                        Direction::Left
                    } else {
                        Direction::Right
                    };
                    self.turn_and_move(dir);
                }

                return self.shoot();
            }
        }
    }
}

pub struct Bullet {
    live: bool,
    x: i32,
    y: i32,
    direction: Direction,
    speed: i32,
    sprite: &'static TwoBppSprite,
    team: Team,
}

impl Bullet {
    pub fn new(
        x: i32,
        y: i32,
        direction: Direction,
        speed: i32,
        team: Team,
    ) -> Bullet {
        Bullet {
            live: true,
            x,
            y,
            direction,
            speed,
            sprite: &BULLET_SPRITE,
            team,
        }
    }

    pub fn collision(&self) -> Collision {
        Collision::square(self.x, self.y, BULLET_WIDTH)
    }

    pub fn is_live(&self) -> bool {
        self.live
    }

    pub fn update(&mut self) {
        let (new_x, new_y) = translate_point(self.x, self.y, self.direction, self.speed);

        if new_x <= BORDER_LEFT
            || new_x >= BORDER_RIGHT - BULLET_WIDTH as i32
            || new_y <= BORDER_TOP
            || new_y >= BORDER_BOTTOM - BULLET_WIDTH as i32
        {
            self.live = false;
        } else {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn hit_enemy(&mut self, enemies: &mut Vec<Tank>) -> Bonus {
        let bullet_ent = self.collision();
        for enemy in enemies.iter_mut() {
            let tank_ent = enemy.collision();
            if self.team != enemy.team && collide(&bullet_ent, &tank_ent) {
                self.live = false;
                enemy.life -= 1;
                if enemy.life <= 0 {
                    enemy.live = false;
                }
                return enemy.model.bonus.clone();
            }
        }
        return Bonus::None;
    }

    pub fn draw(&self) {
        if self.live {
            self.sprite.draw(self.x, self.y, self.direction);
        }
    }
}
