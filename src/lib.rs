#![feature(lazy_cell)]
#![feature(ptr_sub_ptr)]

use fps_config::fps_settings_callback;

pub mod fps_config;
pub mod fps_hooks;
pub mod utils;

pub static mut CURRENT_FPS: i32 = 30;

#[skyline::main(name = "fps_settings_plugin")]
pub fn main() {
    // Adds the setting to the Settings menu, in-game.
    cobapi::install_game_setting(fps_settings_callback);
    // Adds the setting to the Global Settings menu, in the Cobalt menu (located on the title screen).
    cobapi::install_global_game_setting(fps_settings_callback);
    skyline::install_hooks!(
        fps_hooks::vsync_count_hook,
        fps_hooks::get_player_max_speed_hook,
        fps_hooks::get_player_accel_hook,
        fps_hooks::get_player_decel_hook,
        fps_hooks::get_player_rotate_speed_rate_hook
        // fps_hooks::get_player_dash_stop_time_hook
    );
    println!(
        "{}",
        format!("FPS Settings Plugin  {}", env!("CARGO_PKG_VERSION"))
    );
    println!("Source code available at https://github.com/DogeThis/fps-settings-plugin");
}
