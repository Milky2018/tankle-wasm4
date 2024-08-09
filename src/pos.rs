use crate::Direction;

pub fn translate_point(x: i32, y: i32, dir: Direction, speed: i32) -> (i32, i32) {
    match dir {
        Direction::Up => (x, y - speed),
        Direction::Down => (x, y + speed),
        Direction::Left => (x - speed, y),
        Direction::Right => (x + speed, y),
    }
}
