use crate::{fps_config::get_fps_with_default, CURRENT_FPS};
use unity::prelude::*;

#[skyline::hook(offset = 0x250cda0)]
pub fn vsync_count_hook(_: i32, method_info: OptionalMethod) {
    let fps = get_fps_with_default();
    unsafe {
        CURRENT_FPS = fps;
    }

    match fps {
        120 => call_original!(0, method_info), // hidden from menu, set via config---vsync 0 breaks everything...
        60  => call_original!(1, method_info),
        30  => call_original!(2, method_info),
        _   => call_original!(2, method_info) // fallback to 30 fps if invalid setting
    }
}

fn speed_modifier() -> f32 {
    unsafe {
        let speed_mod = 30.0 / CURRENT_FPS as f32;

        // this ensures *most* of the other speed hooks work close to how they would at 30fps
        return speed_mod * speed_mod;
    }
}

// App.HubUtil$$get_PlayerMaxSpeed	7102a5e820	float App.HubUtil$$get_PlayerMaxSpeed(MethodInfo * method)	96
#[unity::hook("App", "HubUtil", "get_PlayerMaxSpeed")]
pub fn get_player_max_speed_hook(method_info: OptionalMethod) -> f32 {
    let speed = call_original!(method_info);

    // the square root makes our speeds normal: 0.25 -> 0.5
    return speed_modifier().sqrt() * speed;
}

#[unity::hook("App", "HubUtil", "get_PlayerAccel")]
pub fn get_player_accel_hook(method_info: OptionalMethod) -> f32 {
    let accel = call_original!(method_info);

    return speed_modifier() * accel;
}

#[unity::hook("App", "HubUtil", "get_PlayerDecel")]
pub fn get_player_decel_hook(method_info: OptionalMethod) -> f32 {
    let decel = call_original!(method_info);

    return speed_modifier() * decel;
}

#[unity::hook("App", "HubUtil", "get_PlayerRotateSpeedRate")]
pub fn get_player_rotate_speed_rate_hook(method_info: OptionalMethod) -> f32 {
    let rotate_rate = call_original!(method_info);

    return speed_modifier().sqrt() * rotate_rate;
}

// #[unity::hook("App", "HubUtil", "get_PlayerDashStopTime")]
// pub fn get_player_dash_stop_time_hook(method_info: OptionalMethod) -> f32 {
//     let dash_stop_time = call_original!(method_info);

//     // return speed_modifier() * dash_stop_time;
//     return dash_stop_time;
// }
