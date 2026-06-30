use engage::{prelude::*, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::{
    utils::save_config,
    utils::localize,
    utils::on_str,
    ACCURATE_MOVEMENT,
};


#[unity::inject(
    namespace = "FPSPlugin",
    name = "MovSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct MovSetting {}

#[unity::injected_methods]
impl MovSetting {
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString { localize("mov_name").into() }
    #[override_virtual(name = "ACall")]
    pub fn a_call(self) -> BasicMenuResult { BasicMenuResult::new() }
    #[override_virtual(name = "BuildAttribute")]
    pub fn build_attribute(self) -> BasicMenuItemAttribute { BasicMenuItemAttribute::enable() }
    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) { self.init_content(); }
    #[override_virtual(name = "InitContent")]
    pub fn init_content(self) {
        let value = *ACCURATE_MOVEMENT.lock().unwrap();
        self.set_title_text(self.get_name());
        self.refresh_text(value);
    }

    #[override_virtual(name = "CustomCall")]
    pub fn custom_call(self) -> BasicMenuResult {
        let value = *ACCURATE_MOVEMENT.lock().unwrap();
        let result = ConfigBasicMenuItem::change_key_value_b(value);

        if value != result {
            *ACCURATE_MOVEMENT.lock().unwrap() = result;
            save_config("mov", result);
            self.refresh_text(result);
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }
}

impl MovSetting {
    pub fn refresh_text(self, value: bool) {
        let help_text = match value {
            true => localize("mov_helptext_on"),
            false => localize("mov_helptext_off"),
        };
        self.set_m_help_text(help_text.into());
        self.set_m_command_text(on_str(value));
        self.update_text();
    }
}

pub fn register_class() -> Class {
    let result = cobapi::injection::register::<MovSetting>();
    match result {
        Ok(t) => t,
        Err(_e) => panic!("Failed to register MovSetting"),
    }
}

#[no_mangle]
pub extern "C" fn mov_settings_callback() -> ConfigBasicMenuItem {
    let instance = MovSetting::instantiate().unwrap();
    instance.try_cast::<ConfigBasicMenuItem>().unwrap()
}

pub fn install() {
    register_class();
    cobapi::install_game_setting(mov_settings_callback);
    cobapi::install_global_game_setting(mov_settings_callback);
}