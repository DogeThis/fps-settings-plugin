use crate::{ACCURATE_MOVEMENT, ACCURATE_SPEED, CURRENT_FPS};
use unity::OptionalMethod;

use engage::app::{HubMoveStateMove, IHubMoveStateMove};

#[skyline::hook(offset = 0x250cda0)]
pub fn vsync_count_hook(_: i32, method_info: OptionalMethod) {
    let vsync = match *CURRENT_FPS.lock().unwrap() {
        120 => 0, // hidden from menu, set via config---vsync 0 breaks everything...
        60  => 1,
        30  => 2,
        _   => 2, // fallback to 30 fps if invalid setting
    };
    call_original!(vsync, method_info);
}

#[skyline::from_offset(0x378EA40)]
fn get_smooth_deltatime() -> f32;

fn get_frametiming() -> f32 {
    return 30.0 * unsafe { get_smooth_deltatime() };
}

fn get_frametiming_static() -> f32 {
    return 30.0 / *CURRENT_FPS.lock().unwrap() as f32;
}

fn speed_modifier() -> f32 {
    match *ACCURATE_SPEED.lock().unwrap() {
        true => get_frametiming(),
        false => 1.0,
    }
}

fn frametime_modifier() -> f32 {
    match *ACCURATE_MOVEMENT.lock().unwrap() {
        true => get_frametiming().powi(2), // squaring ensures *most* of the other speed hooks work close to how they would at 30fps
        false => 1.0,
    }
}

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

static mut HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING: f32 = 2.0;

// to get the actual float values, divide by 1000.
const NPC_MOVE_WALK: i32 = 50;
const NPC_MOVE_WALK_B: i32 = 60;
const NPC_MOVE_GENERIC: i32 = 30;
const NPC_POOL_STOP: i32 = 4;
const NPC_POOL_STOP_B: i32 = 6;
const NPC_POOL_LAUNCH: i32 = 140;
const NPC_POOL_SWIM: i32 = 40;
const NPC_POOL_SWIM_B: i32 = 20;

// floating point pattern helper
fn fpp_helper(float: f32) -> i32 {
    return (float * 1000.0) as i32;
}

#[unity::hook("App", "HubMoveStateMove", "Start")]
pub fn hub_move_state_move_start(this: HubMoveStateMove, resume: bool, method_info: OptionalMethod) {
    let frametiming = get_frametiming_static();
    unsafe {
        // messy initialization
        if HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING == 2.0 {
            HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING = frametiming;
        }

        // fix transition from 60 to 30. currently broken
        if (frametiming - HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING) == 0.5 {
            this.set_m_speed(this.m_speed() / 0.5);
        }
        HUB_MOVE_STATE_MOVE_CURRENT_FRAMETIMING = frametiming;
    }
    
    if frametiming == 1.0 {
        if this.m_speed() == 0.0030002 {
            this.set_m_speed(0.006);
        }
        return call_original!(this, resume, method_info);
    } else {
        match fpp_helper(this.m_speed()) {
            NPC_MOVE_WALK | NPC_MOVE_WALK_B |
            NPC_POOL_SWIM | NPC_POOL_SWIM_B |
            NPC_POOL_STOP | NPC_POOL_LAUNCH => {
                this.set_m_speed(this.m_speed() * frametiming);
            },
            NPC_MOVE_GENERIC if this.m_is_turn() => {
                this.set_m_speed(this.m_speed() * frametiming);
            },
            NPC_POOL_STOP_B => {
                this.set_m_speed(this.m_speed() * frametiming);  // this one does NOT like being an odd number
                this.set_m_speed(this.m_speed() + 0.0000002);
            },
            _ => {}
        }
    }
    
    call_original!(this, resume, method_info);
}

pub fn install() {
    skyline::install_hooks!(
        vsync_count_hook,
        get_player_max_speed_hook,
        get_player_accel_hook,
        get_player_decel_hook,
        get_player_rotate_speed_rate_hook,
        hub_move_state_move_start,
    );
}