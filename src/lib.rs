use std::sync::{LazyLock, Mutex};

use utils::get_config;
mod utils;

mod fps_hooks;
mod fps_config;
mod spd_config;
mod mov_config;

pub static CURRENT_FPS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(60));
pub static ACCURATE_SPEED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));
pub static ACCURATE_MOVEMENT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));

fn load_config() {
    *CURRENT_FPS.lock().unwrap() = get_config("fps", 60);
    *ACCURATE_SPEED.lock().unwrap() = get_config("spd", true);
    *ACCURATE_MOVEMENT.lock().unwrap() = get_config("mov", true);
}

#[skyline::main(name = "fps_settings_plugin")]
pub fn main() {
    load_config();
    fps_hooks::install();
    fps_config::install();
    spd_config::install();
    mov_config::install();
     
    println!("FPS Settings Plugin {}", env!("CARGO_PKG_VERSION"));
    println!("Source code available at https://github.com/DogeThis/fps-settings-plugin");
}