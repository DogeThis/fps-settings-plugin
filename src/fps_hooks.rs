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

// #[unity::hook("App", "HubUtil", "get_PlayerDashStopTime")]
// pub fn get_player_dash_stop_time_hook(method_info: OptionalMethod) -> f32 {
//     let dash_stop_time = call_original!(method_info);

//     // return speed_modifier() * dash_stop_time;
//     return dash_stop_time;
// }


// NPC Handling

// #[skyline::hook(offset = 0x23D3920)]
// fn hub_move_state_move_spline_loop_ctor(
//     this: *const u8,
//     unit: *const u8,
//     data: &Vector3<f32>,
//     body_anim: Il2CppString,
//     face_anim: Il2CppString,
//     is_turn: bool,
//     speed: f32,
//     method_info: OptionalMethod,
// ) {
//     let modified_speed = speed_modifier() * speed;
//     println!("I AM RUNNING!!!!!!!!!!!");

//     call_original!(this, unit, data, body_anim, face_anim, is_turn, modified_speed, method_info);
// }



// #[repr(C)]
// pub struct AppHubMoveStateMoveO {
//     _padding: [u8; 0x3C], // offset
//     m_speed: f32,
// }

// // App.HubMoveStateMove$$IsEnd    71028bfae0    bool App.HubMoveStateMove$$IsEnd(App_HubMoveStateMove_o * __this, MethodInfo * method)    268
// #[unity::hook("App", "HubMoveStateMove", "IsEnd")]
// pub fn hub_move_state_move_is_end(this: *mut AppHubMoveStateMoveO, method_info: OptionalMethod) -> bool {
//     println!("I AM end, i am alive too!!!!!!!!!!!");
//     // unsafe {
//     //     // dereference the raw pointer to access the struct
//     //     let this_ref = &mut *this;

//     //     let result = call_original!(this, method_info);

//     //     this_ref.m_speed *= 0.5;
//     //     result
//     // }
//     return call_original!(this, method_info);
// }