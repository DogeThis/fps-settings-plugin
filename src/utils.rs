use std::fs::{self, OpenOptions};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

use engage::mess::Mess;

use phf::phf_map;

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

static EN_US: phf::Map<&str, &str> = phf_map! {
    "fps_name" => "Frame Rate (FPS)",
    "fps_helptext_30" => "The game's default.",
    "fps_helptext_60" => "Smoother gameplay."
};

static JP: phf::Map<&str, &str> = phf_map! {
    "fps_name" => "フレームレート",
    "fps_helptext_30" => "ゲームのデフォルト",
    "fps_helptext_60" => "より滑らかなゲームプレイ"
};

static EU_FR: phf::Map<&str, &str> = phf_map! {
    "fps_name" => "Fréquence d'image (FPS)",
    "fps_helptext_30" => "La fréquence de base.",
    "fps_helptext_60" => "Une expérience plus fluide."
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
