use std::path::{Path, PathBuf};
use std::str::from_utf8;
use std::{fs, io};

fn get_config_dir() -> PathBuf {
    directories::ProjectDirs::from("com", "Dominik Korsa", "Satori CLI")
        .unwrap()
        .config_dir()
        .to_path_buf()
}

fn ensure_config_dir() {
    fs::create_dir_all(get_config_dir()).unwrap();
}

fn get_username_path() -> PathBuf {
    get_config_dir().join(Path::new("username.txt"))
}

pub fn get_username() -> Option<String> {
    match fs::read_to_string(get_username_path()) {
        Ok(str) => Some(str),
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(err) => panic!("{}", err),
    }
}

pub fn set_username(value: &str) {
    ensure_config_dir();
    fs::write(get_username_path(), value).unwrap();
}
