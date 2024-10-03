use std::fs::{self, OpenOptions};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

use engage::mess::Mess;

use phf::phf_map;
use unity::system::Il2CppString;

pub fn write_to_path(path: &str, data: &str) {
    let path = Path::new(path);
    // Ensure the parent directory exists
    if let Some(parent) = path.parent() {
        if let Err(why) = fs::create_dir_all(parent) {
            panic!(
                "write_to_path couldn't create directory {}: {}",
                parent.display(),
                why
            );
        }
    }

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
    {
        Err(why) => panic!("write_to_path couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!(
            "write_to_path couldn't write to {}: {}",
            path.display(),
            why
        ),
        Ok(_) => (),
    }
}

pub fn read_from_path<T: FromStr>(path: &str) -> Option<T> {
    let path = Path::new(path);
    let mut file = match File::open(&path) {
        Err(why) => {
            println!("read_from_path couldn't open {}: {}", path.display(), why);
            return None;
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("read_from_path couldn't read {}: {}", path.display(), why);
            return None;
        }
        Ok(_) => match s.parse::<T>() {
            Ok(n) => Some(n),
            Err(_) => None,
        },
    }
}

pub const SETTINGS_PATH: &str = "sd:/engage/fps_settings_plugin/";

fn config_path(filename: &str) -> String {
    return format!("{}{}", SETTINGS_PATH, filename);
}

pub fn get_config<T: FromStr>(filename: &str, default_value: T) -> T {
    read_from_path(config_path(filename).as_str()).unwrap_or(default_value)
}

pub fn save_config<T: ToString>(filename: &str, value: T) {
    write_to_path(config_path(filename).as_str(), &value.to_string());
}

pub fn on_str() -> &'static Il2CppString {
    Mess::get("MID_CONFIG_TUTORIAL_ON")
}
pub fn off_str() -> &'static Il2CppString {
    Mess::get("MID_CONFIG_TUTORIAL_OFF")
}

static EN_US: phf::Map<&str, &str> = phf_map! {
    "fps_name" => "Frame Rate (FPS)",
    "fps_helptext_30" => "The game's default.",
    "fps_helptext_60" => "Smoother gameplay.",
    "mov_name" => "Accurate Movement",
    "mov_helptext_off" => "Do not use accurate movement above 30 FPS.",
    "mov_helptext_on" => "Use accurate movement above 30 FPS.",
    "spd_name" => "Accurate Speed",
    "spd_helptext_off" => "Do not adjust movement speed at high frame rates.",
    "spd_helptext_on" => "Adjust movement speed at high frame rates."
};

static JP: phf::Map<&str, &str> = phf_map! {
    "fps_name" => "フレームレート (FPS)",
    "fps_helptext_30" => "ゲームのデフォルト",
    "fps_helptext_60" => "より滑らかなゲームプレイ",
    "mov_name" => "正確な動き",
    "mov_helptext_off" => "30FPS以上の時、正確な動きを使用しません",
    "mov_helptext_on" => "30FPS以上の時、正確な動きを使用します",
    "spd_name" => "正確なスピード",
    "spd_helptext_off" => "高フレームレートでの移動スピードを調整しません",
    "spd_helptext_on" => "高フレームレートでの移動スピードを調整します"
};

static EU_FR: phf::Map<&str, &str> = phf_map! {
    "fps_name" => "Fréquence d'image (FPS)",
    "fps_helptext_30" => "La fréquence de base.",
    "fps_helptext_60" => "Une expérience plus fluide.",
    "mov_name" => "Accurate Movement",
    "mov_helptext_off" => "N'utilisez pas de mouvement précis au-dessus de 30 FPS.",
    "mov_helptext_on" => "Utilisez des mouvements précis au-dessus de 30 FPS.",
    "spd_name" => "Accurate Speed",
    "spd_helptext_off" => "Do not adjust movement speed at high frame rates.",
    "spd_helptext_on" => "Adjust movement speed at high frame rates."
};

pub fn localize(key: &str) -> String {
    let map: &phf::Map<&str, &str> = match Mess::get_language_directory_name()
        .get_string()
        .unwrap()
        .to_lowercase()
        .as_str()
    {
        "us/usen" => &EN_US,
        "jp/jpja" => &JP,
        "eu/eufr" => &EU_FR,
        // Fallback to English, sorry if you're not in one of these regions
        _ => &EN_US,
    };
    return map[key].to_string();
}
