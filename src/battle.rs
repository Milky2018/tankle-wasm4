use oorandom::Rand32;

use crate::bonus::*;
use crate::collide::*;
use crate::enemy::*;
use crate::level::*;
use crate::scene::*;
use crate::sprite::*;
use crate::tank::*;
use crate::ui;

pub struct Battle {
    frame_count: u32,
    bullets: Vec<Bullet>,
    tanks: Vec<Tank>,
    rng: Rand32,
    levels: Vec<Level>,
    level_idx: usize,
    bonus: bool,
}

impl Battle {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            bullets: Vec::new(),
            tanks: vec![Tank::new(60, 60, Direction::Up, TANK1, Team::Players)],
            rng: oorandom::Rand32::new(42),
            levels: Level::make_levels(),
            level_idx: 0,
            bonus: false,
        }
    }

    pub fn reset(&mut self, level: usize) {
        self.frame_count = 0;
        self.bullets.clear();
        self.tanks.clear();
        self.tanks
            .push(Tank::new(60, 60, Direction::Up, TANK1, Team::Players));
        self.level_idx = level;
        self.levels = Level::make_levels();
    }
}

impl Scene for Battle {
    fn update(&mut self) -> usize {
        self.frame_count += 1;

        let poses: Vec<Collision> = self
            .tanks
            .iter()
            .map(|tank| tank.collision())
            .chain(self.bullets.iter().map(|bullet| bullet.collision()))
            .collect();

        let mut bonus_gained = Vec::new();

        if self.bonus {
            if self
                .tanks
                .iter()
                .filter(|tank| tank.get_team() == Team::Bonus)
                .count() < 4
            {
                self.tanks.retain(|tank| tank.get_team() != Team::Bonus);
                self.bonus = false;
            }
        } else {
            if let Some(level) = self.levels.get_mut(self.level_idx) {
                match level.update(&poses) {
                    SpawnResult::Over => {
                        if self
                            .tanks
                            .iter()
                            .all(|tank| tank.get_team() == Team::Players)
                        {
                            self.level_idx += 1;
                            bonus_gained.push(Bonus::Refill);
                            self.bonus = true;
                            self.tanks.extend(make_bonus());
                            if self.level_idx >= self.levels.len() {
                                return SUCCESS_SCENE;
                            }
                        }
                    }
                    SpawnResult::Spawn(tank) => {
                        self.tanks.push(tank);
                    }
                    _ => {}
                }
            } else {
                return SUCCESS_SCENE;
            }
        }

        for tank in self.tanks.iter_mut() {
            if let Option::Some(bullet) = tank.control(&mut self.rng) {
                self.bullets.push(bullet);
            }
        }

        if self.frame_count % 2 == 0 {
            let collisions: Vec<Collision> = self
                .tanks
                .iter()
                .filter(|tank| tank.get_team() != Team::Bonus)
                .map(|tank| tank.collision())
                .collect();
            for tank in self.tanks.iter_mut() {
                tank.update(&collisions);
            }

            for bullet in self.bullets.iter_mut() {
                bullet.update();
                bonus_gained.push(bullet.hit_enemy(&mut self.tanks));
            }

            self.tanks.retain(|enemy| enemy.is_live());
            self.bullets.retain(|bul| bul.is_live());
        }
        for enemy in self.tanks.iter() {
            enemy.draw();
        }
        for bullet in self.bullets.iter() {
            bullet.draw();
        }

        if let Option::Some(player) = self
            .tanks
            .iter_mut()
            .find(|tank| tank.get_team() == Team::Players)
        {
            for bonus in bonus_gained.iter() {
                bonus.up(player);
            }
            let life = player.get_life();
            ui::panel(player.get_model(), life, self.level_idx);
            return BATTLE_SCENE;
        } else {
            self.reset(self.level_idx);
            return DEFEAT_SCENE;
        }
    }
}
