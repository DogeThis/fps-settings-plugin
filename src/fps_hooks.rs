use crate::{ACCURATE_MOVEMENT, ACCURATE_SPEED, CURRENT_FPS};
use unity::prelude::*;

#[skyline::hook(offset = 0x250cda0)]
pub fn vsync_count_hook(_: i32, method_info: OptionalMethod) {
    let vsync = match unsafe { CURRENT_FPS } {
        120 => 0, // hidden from menu, set via config---vsync 0 breaks everything...
        60  => 1,
        30  => 2,
        _   => 2, // fallback to 30 fps if invalid setting
    };

    call_original!(vsync, method_info);
}

fn get_frametiming() -> f32 {
    return 30.0 / unsafe { CURRENT_FPS } as f32;
}

fn speed_modifier() -> f32 {
    match unsafe { ACCURATE_SPEED } {
        true => get_frametiming(),
        false => 1.0,
    }
}

fn frametime_modifier() -> f32 {
    match unsafe { ACCURATE_MOVEMENT } {
        true => get_frametiming().powi(2), // squaring ensures *most* of the other speed hooks work close to how they would at 30fps
        false => 1.0,
    }
}

// App.HubUtil$$get_PlayerMaxSpeed	7102a5e820	float App.HubUtil$$get_PlayerMaxSpeed(MethodInfo * method)	96
#[unity::hook("App", "HubUtil", "get_PlayerMaxSpeed")]
pub fn get_player_max_speed_hook(method_info: OptionalMethod) -> f32 {
    let speed = call_original!(method_info);

    return speed_modifier() * speed;
}

#[unity::hook("App", "HubUtil", "get_PlayerAccel")]
pub fn get_player_accel_hook(method_info: OptionalMethod) -> f32 {
    let accel = call_original!(method_info);

    return frametime_modifier() * accel;
}

#[unity::hook("App", "HubUtil", "get_PlayerDecel")]
pub fn get_player_decel_hook(method_info: OptionalMethod) -> f32 {
    let decel = call_original!(method_info);

    return frametime_modifier() * decel;
}

#[unity::hook("App", "HubUtil", "get_PlayerRotateSpeedRate")]
pub fn get_player_rotate_speed_rate_hook(method_info: OptionalMethod) -> f32 {
    let rotate_rate = call_original!(method_info);

    return frametime_modifier().sqrt() * rotate_rate;
}

// NPC Handling

#[repr(C)]
pub struct AppHubMoveStateMoveO {
    _padding1: [u8; 0x10],
    _padding2: [u8; 0x8],
    m_body_anim: Box<str>,
    m_face_anim: Box<str>,
    m_is_turn: bool,
    m_resume: bool,
    _padding3: [u8; 2],
    m_speed: f32,
    m_blend: f32,
}

static mut HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING: f32 = 2.0;

// floating point pattern helper
fn fpp_helper(float: f32) -> i32 {
    return (float * 1000.0) as i32;
}

#[unity::hook("App", "HubMoveStateMove", "Start")]
pub fn hub_move_state_move_start(this: &mut AppHubMoveStateMoveO, resume: bool, method_info: OptionalMethod) {
    let frametiming = get_frametiming();
    unsafe {
        // messy initialization
        if HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING == 2.0 {
            HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING = frametiming;
        }

        // fix transition from 60 to 30. currently broken
        if (frametiming - HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING) == 0.5 {
            this.m_speed /= 0.5;
        }
        HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING = frametiming;
    }
    
    if frametiming == 1.0 {
        if this.m_speed == 0.0030002 {
            this.m_speed = 0.006;
        }
        println!("NEW Speed: {} {}", this.m_speed, this.m_is_turn);
        return call_original!(this, resume, method_info);
    } else {
        match fpp_helper(this.m_speed) {
            50 | 60 => { // NPC Walk
                this.m_speed *= frametiming;
            },
            30 if this.m_is_turn => {
                this.m_speed *= frametiming;
            },
            4 => { // NPC Pool Stop
                this.m_speed *= frametiming;
            },
            6 => { // NPC Pool Stop
                this.m_speed *= frametiming;  // this one does NOT like being an odd number
                this.m_speed += 0.0000002;
            },
            140 => { // NPC Pool Kick-off
                this.m_speed *= frametiming;
            },
            40 => { // NPC Pool
                this.m_speed *= frametiming;
            },
            20 => { // NPC Pool
                this.m_speed *= frametiming;
            },
            _ => {}
        }
    }
    
    call_original!(this, resume, method_info);
    println!("NEW Speed: {} {}", this.m_speed, this.m_is_turn);
}