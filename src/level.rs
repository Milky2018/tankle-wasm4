use crate::tank::*;
use crate::collide::*;
use crate::enemy::*;

pub struct Level {
    enemies: MultiQueue<TankModel>,
    spawn_cd: u32,
    frame_count: u32,
}

struct MultiNode<T> {
    data: T, 
    num: usize,
}

struct MultiQueue<T> {
    queue: Vec<MultiNode<T>>,
}

impl<T: Clone> MultiQueue<T> {
    pub fn new(nodes: Vec<(T, usize)>) -> Self {
        let mut queue = Vec::new();
        for node in nodes {
            let node = MultiNode {
                data: node.0,
                num: node.1,
            };
            queue.push(node);
        }
        Self {
            queue
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.retain(|node| node.num > 0);

        if let Some(node) = self.queue.first_mut() {
            node.num -= 1;
            return Some(node.data.clone());
        } else {
            None
        }
    }
}

pub enum SpawnResult {
    Spawn(Tank),
    Over,
    Occupied,
    Heat,
}

impl Level {
    pub fn make_levels() -> Vec<Level> {
        let level1 = Level {
            enemies: MultiQueue::new(vec![(TANK2, 5)]),
            spawn_cd: 60,
            frame_count: 0,
        };

        let level2 = Level {
            enemies: MultiQueue::new(vec![
                (TANK2, 3),
                (TANK3, 1),
                (TANK2, 3),
            ]),
            spawn_cd: 40, 
            frame_count: 0
        };

        let level3 = Level {
            enemies: MultiQueue::new(vec![
                (TANK3, 4),
                (TANK2, 4),
                (TANK3, 4),
            ]),
            spawn_cd: 40, 
            frame_count: 0
        };

        let level4 = Level {
            enemies: MultiQueue::new(vec![
                (TANK2, 20),
            ]),
            spawn_cd: 15, 
            frame_count: 0
        };

        vec![
            level1,
            level2,
            level3,
            level4,
        ]
    }

    pub fn update(&mut self, occupied: &Vec<Collision>) -> SpawnResult {
        self.frame_count += 1;
        if self.frame_count >= self.spawn_cd {
            if let Option::Some((x, y, dir)) = spawn_enemy_pos(occupied) {
                if let Option::Some(model) = self.enemies.pop() {
                    self.frame_count -= self.spawn_cd;
                    return SpawnResult::Spawn(Tank::new(x, y, dir, model, Team::Enemies));
                } else {
                    return SpawnResult::Over;
                }
            } else {
                return SpawnResult::Occupied;
            }
        } else {
            return SpawnResult::Heat;
        }
    }
}