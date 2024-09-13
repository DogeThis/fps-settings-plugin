use crate::{fps_config::get_fps_with_default, CURRENT_FPS};
use unity::prelude::*;

#[skyline::hook(offset = 0x250cda0)]
pub fn vsync_count_hook(_: i32, method_info: OptionalMethod) {
    let fps = get_fps_with_default();
    unsafe {
        CURRENT_FPS = fps;
    }
    if fps == 60 {
        call_original!(1, method_info);
    } else {
        call_original!(2, method_info)
    }
}

// App.HubUtil$$get_PlayerMaxSpeed	7102a5e820	float App.HubUtil$$get_PlayerMaxSpeed(MethodInfo * method)	96
#[skyline::hook(offset = 0x2a5e820)]
pub fn get_player_max_speed_hook(method_info: OptionalMethod) -> f32 {
    let speed = call_original!(method_info);
    unsafe {
        if CURRENT_FPS == 30 {
            return speed;
        } else {
            // don't let the player go too fast when the game is running at 60fps
            return speed / 2.0;
        }
    }
}
