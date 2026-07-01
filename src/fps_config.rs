use engage::{prelude::*, app::BasicMenu_Result, app::basicmenuitem::BasicMenuItem_Attribute, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::{
    fps_hooks::vsync_count_hook,
    utils::save_config,
    utils::localize,
    CURRENT_FPS,
};

#[unity::inject(
    namespace = "FPSPlugin",
    name = "FPSSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct FpsSetting {}

#[unity::injected_methods]
impl FpsSetting {
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString { localize("fps_name").into() }
    #[override_virtual(name = "ACall")]
    pub fn a_call(self) -> BasicMenu_Result { BasicMenu_Result::pass() }
    #[override_virtual(name = "BuildAttribute")]
    pub fn build_attribute(self) -> BasicMenuItem_Attribute { BasicMenuItem_Attribute::enable() }
    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) { self.init_content(); }
    #[override_virtual(name = "InitContent")]
    pub fn init_content(self) {
        let value = *CURRENT_FPS.lock().unwrap();
        self.set_title_text(self.get_name());
        self.refresh_text(value);
    }

    #[override_virtual(name = "CustomCall")]
    pub fn custom_call(self) -> BasicMenu_Result {
        let value = *CURRENT_FPS.lock().unwrap();
        let result = ConfigBasicMenuItem::change_key_value(value, 30, 60, 30);

        if value != result {
            *CURRENT_FPS.lock().unwrap() = result;
            vsync_count_hook(0, None);
            save_config("fps", result);
            self.refresh_text(result);
            BasicMenu_Result::se_cursor()
        } else {
            BasicMenu_Result::pass()
        }
    }
}

impl FpsSetting {
    pub fn refresh_text(self, fps: i32) {
        let help_text = match fps {
            30 => localize("fps_helptext_30"),
            60 => localize("fps_helptext_60"),
            _ => "How did you set this...?".to_string(),
        };
        self.set_m_help_text(help_text.into());
        self.set_m_command_text(fps.to_string().into());
        self.update_text();
    }
}

pub fn register_class() -> Class {
    let result = cobapi::injection::register::<FpsSetting>();
    match result {
        Ok(t) => t,
        Err(_e) => panic!("Failed to register FpsSetting"),
    }
}

#[no_mangle]
pub extern "C" fn fps_settings_callback() -> ConfigBasicMenuItem {
    let instance = FpsSetting::instantiate().unwrap();
    instance.try_cast::<ConfigBasicMenuItem>().unwrap()
}

pub fn install() {
    register_class();
    cobapi::install_game_setting(fps_settings_callback);
    cobapi::install_global_game_setting(fps_settings_callback);
}