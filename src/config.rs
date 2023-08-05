// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

extern crate xdg;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub misses: usize,
    pub startspeed: u64,
    pub minwords: usize,
    pub maxwords: usize,
    pub data: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut data = match xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            Ok(xdg_base_dirs) => {
                let mut path = xdg_base_dirs.get_data_dirs();
                path.push(xdg_base_dirs.get_data_home());
                path
            }
            Err(_) => vec![],
        };
        #[cfg(feature = "devel")]
        data.insert(0, PathBuf::from(format!("{}/data", env!("CARGO_MANIFEST_DIR"))));
        Config {
            misses: 10,
            startspeed: 0,
            minwords: 1,
            maxwords: 20,
            data: data,
        }
    }
}

impl Config {
    pub fn read() -> Config {
        match xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            Ok(xdg_dirs) => {
                if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
                    let config_content = fs::read_to_string(&config_path).unwrap_or_default();
                    match toml::from_str(&config_content) {
                        Ok(config) => return config,
                        Err(e) => eprintln!("Could not parse config file: {}", e),
                    }
                } else {
                    //for now disabled, should only be shown with some kind of --debug flag
                    //eprintln!("Could not load configuration file, using default settings.");
                }
            }
            Err(e) => eprintln!("Cannot determine XDG base directories: {}", e),
        }
        Config::default()
    }
}

impl From<std::path::PathBuf> for Config {
    fn from(path: std::path::PathBuf) -> Self {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
        Config::read()
    }
}
