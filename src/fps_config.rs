use engage::menu::{
    config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods},
    BasicMenuResult,
};
use unity::prelude::*;

use crate::{
    fps_hooks::vsync_count_hook,
    utils::{localize, read_from_path, write_to_path},
    CURRENT_FPS,
};
pub struct FPSSetting;
pub const FPS_PATH: &str = "sd:/engage/fps_settings_plugin/fps";

impl ConfigBasicMenuItemSwitchMethods for FPSSetting {
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        unsafe {
            CURRENT_FPS = get_fps_with_default();
        };
    }

    extern "C" fn custom_call(
        this: &mut ConfigBasicMenuItem,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        let current_fps = unsafe { CURRENT_FPS };

        let result = ConfigBasicMenuItem::change_key_value_i(current_fps, 30, 60, 30);

        if current_fps != result {
            save_fps(result);
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        unsafe {
            this.command_text = format!("{:}", CURRENT_FPS).into();
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { CURRENT_FPS } {
            30 => this.help_text = localize("fps_helptext_30").into(),
            60 => this.help_text = localize("fps_helptext_60").into(),
            _ => this.help_text = "How did you set this framerate...".into(),
        }
    }
}

pub fn get_fps() -> Option<i32> {
    read_from_path(FPS_PATH)
}

pub fn get_fps_with_default() -> i32 {
    get_fps().unwrap_or(30)
}

pub fn save_fps(fps: i32) {
    unsafe {
        CURRENT_FPS = fps;
    }
    write_to_path(FPS_PATH, &format!("{:}", fps));
    vsync_count_hook(fps, None);
}

#[no_mangle] // no_mangle is an attribute used to ask Rust not to modify your function name to facilitate communication with code from other sources.
pub extern "C" fn fps_settings_callback() -> &'static mut ConfigBasicMenuItem {
    // Your callback must return a ConfigBasicMenu, which you can acquire by using new_gauge or new_switch.
    ConfigBasicMenuItem::new_switch::<FPSSetting>("FPS Settings")
}
