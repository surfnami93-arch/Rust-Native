use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSystem {
    history: Vec<String>,
}

impl JsonSystem {
    pub fn save_state(state: &Self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(state).unwrap();
        fs::write("state.json", json)
    }
    pub fn load_state() -> Self {
        match fs::read_to_string("state.json") {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Self {
                history: Vec::new(),
            }),
            Err(_) => Self {
                history: Vec::new(),
            },
        }
    }
}

pub fn dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,                             // 成功: PathBuf を取り出して返す
        Err(e) => panic!("current_dir error: {}", e), // 失敗: panic
    }
}

use std::io::{Error, ErrorKind};
pub mod command_utils {
    use super::*;
    pub fn cargo_search_command() -> std::io::Result<std::path::PathBuf> {
        let cmd_name = "cargo"; // 最初の要素だけ取り出す(消費される)
        let path = std::env::var("PATH")
            .unwrap_or_default()
            .split(';')
            .find_map(|dir| {
                let candidate = std::path::Path::new(dir)
                    .join(cmd_name)
                    .with_extension("exe");
                candidate.exists().then_some(candidate)
            });

        match path {
            Some(p) => {
                return Ok(p);
            }
            None => println!("cargo not install"),
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "Please install Cargo.
                Since Rust Native is built on Cargo, it won't work without Cargo."
                .to_string(),
        ));
    }
}
