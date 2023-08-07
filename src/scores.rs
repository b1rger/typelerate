// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

extern crate xdg;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use std::path::PathBuf;
use crate::context::Context;
use crate::common::fileextensions::FileExtensions;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Score {
    pub name: String,
    pub points: f32,
    pub speed: u64,
    pub width: u16,
    pub height: u16,
    pub failed: usize,
    pub wordfile: PathBuf,
    pub timestamp: SystemTime,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            name: String::from("No name"),
            points: 0.0,
            speed: 0,
            width: 0,
            height: 0,
            failed: 0,
            wordfile: PathBuf::from("/dev/null"),
            timestamp: SystemTime::now()
        }
    }
}

impl From<&Context> for Score {
    fn from(ctx: &Context) -> Self {
        let binding = PathBuf::from("/dev/null");
        let wordfile = match &ctx.wordfile {
            Some(file) => file,
            None => &binding
        };
        let name = match &ctx.name {
            Some(name) => name.to_string(),
            None => String::from("noname"),
        };
        Score {
            points: ctx.points,
            speed: ctx.speed,
            width: ctx.width,
            height: ctx.height,
            failed: ctx.failed,
            wordfile: wordfile.to_path_buf(),
            name: name,
            ..Default::default()
        }
    }
}

impl From<&Score> for String {
    fn from(score: &Score) -> String {
        let title = score.wordfile.firstline().unwrap_or_default();
        let name = if score.name.is_empty() {
            String::from("No name")
        } else {
            score.name.clone()
        };
        format!("{}: {:.3} [{}]", name, score.points, title)
    }
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Scores {
    pub scores: Vec<Score>
}

impl Default for Scores {
    fn default() -> Self {
        Scores {
            scores: vec![]
        }
    }
}

impl From<Scores> for String {
    fn from(scores: Scores) -> String {
        let scores: Vec<String> = scores.scores.iter().map(|score| score.into()).collect();
        scores.join("\n")
    }
}

impl Scores {
    pub fn read() -> Scores {
        if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            if let Some(scores_path) = xdg_dirs.find_state_file("scores.toml") {
                let content = fs::read_to_string(&scores_path).unwrap_or_default();
                match toml::from_str(&content) {
                    Ok(scores) => return scores,
                    Err(e) => eprintln!("Could not parse config file: {}", e),
                }
            }
        }
        Scores::default()
    }

    pub fn write(&mut self, max: usize) -> bool {
        self.scores.sort_by_key(|score| score.points as u32);
        self.scores.reverse();
        self.scores.truncate(max);

        if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            if let Ok(scores_path) = xdg_dirs.place_state_file("scores.toml") {
                if let Ok(scores) = toml::to_string(&self) {
                    if let Ok(mut file) = File::create(scores_path) {
                        writeln!(&mut file, "{}", scores);
                        return true;
                    }
                }
            }
        }
        false
    }
}
