// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use rand::seq::IteratorRandom;
use std::fs::File;
use std::io::BufRead;
use std::io::{Error, ErrorKind};

pub trait FileExtensions {
    fn valid(&self) -> bool;
    fn firstline(&self) -> Result<String, std::io::Error>;
    fn randomline(&self) -> Result<String, std::io::Error>;
}

impl FileExtensions for std::path::PathBuf {
    fn valid(&self) -> bool {
        if let Ok(file) = File::open(self) {
            return file.valid();
        }
        false
    }
    fn firstline(&self) -> Result<String, std::io::Error> {
        let file = File::open(self)?;
        file.firstline()
    }
    fn randomline(&self) -> Result<String, std::io::Error> {
        let file = File::open(self)?;
        file.randomline()
    }
}

impl FileExtensions for std::fs::File {
    fn valid(&self) -> bool {
        std::io::BufReader::new(self).lines().all(|x| x.is_ok())
    }
    fn firstline(&self) -> Result<String, std::io::Error> {
        if let Some(line) = std::io::BufReader::new(self).lines().next() {
            return line;
        }
        Err(Error::new(
            ErrorKind::Other,
            "Error getting first line of file.",
        ))
    }
    fn randomline(&self) -> Result<String, std::io::Error> {
        let mut rng = rand::thread_rng();
        if let Some(line) = std::io::BufReader::new(self)
            .lines()
            .skip(1)
            .choose(&mut rng)
        {
            return line;
        }
        Err(Error::new(
            ErrorKind::Other,
            "Error getting random line of file.",
        ))
    }
}
