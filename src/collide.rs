#[derive(Clone, PartialEq)]
pub struct Collision {
    x: i32, 
    y: i32, 
    width: u32,
    height: u32,
}

pub fn collide(ent1: &Collision, ent2: &Collision) -> bool {
    ent1.x < ent2.x + ent2.width as i32 && 
    ent2.x < ent1.x + ent1.width as i32 && 
    ent1.y < ent2.y + ent2.height as i32 && 
    ent2.y < ent1.y + ent1.height as i32
}

impl Collision {
    pub fn rect(x: i32, y: i32, width: u32, height: u32) -> Collision {
        Collision { x, y, width, height }
    }

    pub fn square(x: i32, y: i32, size: u32) -> Collision {
        Collision { x, y, width: size, height: size }
    }
}