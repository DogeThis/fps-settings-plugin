use unity::prelude::*;

use engage::menu::{
    BasicMenuResult,
    config::{
        ConfigBasicMenuItem,
        ConfigBasicMenuItemSwitchMethods
    }
};

use crate::{
    utils::{get_config, localize, off_str, on_str, save_config},
    ACCURATE_MOVEMENT, ACCURATE_SPEED,
};

pub struct MovSetting;

impl ConfigBasicMenuItemSwitchMethods for MovSetting {
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        unsafe {
            ACCURATE_MOVEMENT = get_config("mov", true);
        };
    }

    extern "C" fn custom_call(
        this: &mut ConfigBasicMenuItem,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        let accurate_movement = unsafe { ACCURATE_MOVEMENT };

        let result = ConfigBasicMenuItem::change_key_value_b(accurate_movement);

        if accurate_movement != result {
            unsafe { ACCURATE_MOVEMENT = result };
            save_config("mov", result);
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { ACCURATE_MOVEMENT } {
            true => this.command_text = on_str(),
            false => this.command_text = off_str(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { ACCURATE_MOVEMENT } {
            true => this.help_text = localize("mov_helptext_on").into(),
            false => this.help_text = localize("mov_helptext_off").into(),
        }
    }
}

pub struct SpdSetting;

impl ConfigBasicMenuItemSwitchMethods for SpdSetting {
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        unsafe {
            ACCURATE_SPEED = get_config("spd", true);
        };
    }

    extern "C" fn custom_call(
        this: &mut ConfigBasicMenuItem,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        let accurate_speed = unsafe { ACCURATE_SPEED };

        let result = ConfigBasicMenuItem::change_key_value_b(accurate_speed);

        if accurate_speed != result {
            unsafe { ACCURATE_SPEED = result };
            save_config("spd", result);
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { ACCURATE_SPEED } {
            true => this.command_text = on_str(),
            false => this.command_text = off_str(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        match unsafe { ACCURATE_SPEED } {
            true => this.help_text = localize("spd_helptext_on").into(),
            false => this.help_text = localize("spd_helptext_off").into(),
        }
    }
}

#[no_mangle] // no_mangle is an attribute used to ask Rust not to modify your function name to facilitate communication with code from other sources.
pub extern "C" fn mov_settings_callback() -> &'static mut ConfigBasicMenuItem {
    // Your callback must return a ConfigBasicMenu, which you can acquire by using new_gauge or new_switch.
    ConfigBasicMenuItem::new_switch::<MovSetting>(localize("mov_name"))
}

#[no_mangle]
pub extern "C" fn spd_settings_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<SpdSetting>(localize("spd_name"))
}