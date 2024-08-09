use crate::wasm4::*;

#[derive(Debug, Clone, Copy)]
pub struct TwoBppSprite {
    data: &'static [u8],
    width: u32,
    height: u32
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl TwoBppSprite {
    pub const fn new(data: &'static [u8], width: u32, height: u32) -> Self {
        if data.len() as u32 * (u8::BITS as u32) / 2 < width * height {
            panic!("Insufficient data for sprite size");
        }
        Self {
            data,
            width,
            height
        }
    }

    pub fn draw(&self, x: i32, y: i32, dir: Direction) {
        let flip_y = if dir == Direction::Down || dir == Direction::Right { BLIT_FLIP_Y } else { 0 };
        let rotate = if dir == Direction::Left || dir == Direction::Right { BLIT_ROTATE } else { 0 };
        unsafe { *DRAW_COLORS = 0x4230 }
        blit(self.data, x, y, self.width, self.height, 
            BLIT_2BPP | flip_y | rotate);
    }
}

#[macro_export]
macro_rules! make_sprite {
    (($width:expr,$height:expr) $($pixel:tt)*) => {{
        const DATA: [u8; ($width * $height) as usize] = {
            let mut data = [0; ($width * $height) as usize];
            let mut data_index = 0;
            let mut bit_index = 4;
            $(
                bit_index -= 1;
                data[data_index] |= make_sprite!(@pixel $pixel) << (bit_index * 2);
                if bit_index == 0 {
                    bit_index = 4;
                    data_index += 1;
                }
            )*
            let _ = data_index;
            let _ = bit_index;
            data
        };
        TwoBppSprite::new(&DATA, $width, $height)
    }};

    (@pixel .) => { 0 };

    (@pixel x) => { 1 };

    (@pixel +) => { 2 };

    (@pixel o) => { 3 };
}

pub const TANK_WIDTH: u32 = 12;

pub const BULLET_WIDTH: u32 = 4;

pub const BULLET_SPRITE: TwoBppSprite = 
    make_sprite! {
        (BULLET_WIDTH, BULLET_WIDTH)
        . + + .
        + + + +
        + + + +
        . . . .
    };