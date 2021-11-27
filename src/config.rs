use cookie_store::CookieStore;
use directories::ProjectDirs;
use serde::Deserialize;
use std::io::BufReader;
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Deserialize, Debug)]
struct Environment {
    satori_username: Option<String>,
    satori_password: Option<String>,
}

fn parse_env() -> Environment {
    envy::from_env().unwrap()
}

fn get_project_dirs() -> ProjectDirs {
    directories::ProjectDirs::from("com", "Dominik Korsa", "Satori CLI").unwrap()
}

fn ensure_config_dir() -> PathBuf {
    let config_dir = get_project_dirs().config_dir().to_path_buf();
    fs::create_dir_all(&config_dir).unwrap();
    config_dir.to_path_buf()
}

fn ensure_data_dir() -> PathBuf {
    let data_dir = get_project_dirs().data_dir().to_path_buf();
    fs::create_dir_all(&data_dir).unwrap();
    data_dir.to_path_buf()
}

fn get_username_path() -> PathBuf {
    ensure_config_dir().join(Path::new("username.txt"))
}

fn get_cookie_path() -> PathBuf {
    ensure_data_dir().join(Path::new("cookies.json"))
}

pub fn get_username() -> Option<String> {
    let env = parse_env();
    if let Some(username) = env.satori_username {
        return Some(username);
    }
    match fs::read_to_string(get_username_path()) {
        Ok(str) => Some(str),
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(err) => panic!("{}", err),
    }
}

pub fn set_username(value: &str) {
    fs::write(get_username_path(), value).unwrap();
}

pub fn get_password() -> Option<String> {
    let env = parse_env();
    env.satori_password
}

pub fn load_cookies() -> CookieStore {
    match fs::File::open(get_cookie_path()) {
        Ok(file) => CookieStore::load_json(BufReader::new(file)).unwrap(),
        Err(err) if err.kind() == NotFound => CookieStore::default(),
        Err(err) => panic!("{}", err),
    }
}

pub fn save_cookies(store: &mut CookieStore) {
    let mut writer = fs::File::create(get_cookie_path())
        .map(io::BufWriter::new)
        .unwrap();
    store.save_json(&mut writer).unwrap();
}
