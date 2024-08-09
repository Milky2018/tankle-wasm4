use crate::scene::*;
use crate::tank::*;
use crate::wasm4::*;

fn life_sripe(maxhp: i32, life: i32) {
    unsafe { *DRAW_COLORS = 0x3 }
    text(b"LIFE", 6, 4);

    let length = (maxhp * 8 + 4) as u32;
    rect(40, 3, length, 10);

    let length = (life * 8) as u32;
    unsafe { *DRAW_COLORS = 0x2 }
    rect(42, 5, length, 6);
}

pub fn panel(model: &TankModel, life: i32, lv: usize) {
    unsafe { *DRAW_COLORS = 0x2 }
    rect(120, 0, 40, 160);

    unsafe { *DRAW_COLORS = 0x2 }
    rect(0, 0, 120, 16);

    life_sripe(model.maxhp, life);

    unsafe { *DRAW_COLORS = 0x3 }

    text(b"SPEED", 122, 16);
    let speed_text = format!("{}", model.engine);
    text(speed_text.as_bytes(), 130, 26);

    text(b"TRAJ", 122, 56);
    let traj_text = format!("{}", model.traj);
    text(traj_text.as_bytes(), 130, 66);

    text(b"CD", 122, 96);
    let cd_text = format!("{}", model.cooldown);
    text(cd_text.as_bytes(), 130, 106);

    text(b"LV", 122, 136);
    let lv_text = format!("{}", lv + 1);
    text(lv_text.as_bytes(), 130, 146);
}

pub struct Welcome;

impl Scene for Welcome {
    fn update(&mut self) -> usize {
        unsafe { *DRAW_COLORS = 0x2 }
        rect(0, 0, 160, 160);
    
        unsafe { *DRAW_COLORS = 0x3 }
        text(b"TANKLE", 30, 10);
        text(b"\x80  Start", 30, 50);

        let gamepad = unsafe { *GAMEPAD1 };
        if gamepad & BUTTON_1 != 0 {
            return BATTLE_SCENE;
        } else {
            return WELCOME_SCENE;
        }
    }
}

pub struct Victory;

impl Scene for Victory {
    fn update(&mut self) -> usize {
        unsafe { *DRAW_COLORS = 0x2 }
        rect(0, 0, 160, 160);
    
        unsafe { *DRAW_COLORS = 0x3 }
        text(b"Victory", 30, 10);

        SUCCESS_SCENE
    }
}

pub struct Defeat;

impl Scene for Defeat {
    fn update(&mut self) -> usize {
        unsafe { *DRAW_COLORS = 0x2 }
        rect(0, 0, 160, 160);
    
        unsafe { *DRAW_COLORS = 0x3 }
        text(b"Defeat", 30, 10);

        // text(b"\x80  Continue", 30, 50);

        // let gamepad = unsafe { *GAMEPAD1 };
        // if gamepad & BUTTON_1 != 0 {
        //     return BATTLE_SCENE;
        // }

        DEFEAT_SCENE
    }
}