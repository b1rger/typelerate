// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::cli::Cli;
use crate::common::FileExtensions;
use crate::config::Config;
use crate::scores::Scores;
use clap::Parser;
use rand::Rng;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Run,
    Pause,
    Quit,
    Score,
    Help,
}

pub struct Context {
    pub config: Config,
    pub name: Option<String>,
    pub wordfile: Option<PathBuf>,
    pub wordlist: Vec<Word>,
    pub points: f32,
    pub speed: u64,
    pub last_tick: SystemTime,
    pub chars: Vec<char>,
    pub failed: usize,
    pub state: State,
    pub last_state: State,
    pub width: u16,
    pub height: u16,
}

impl Default for Context {
    fn default() -> Self {
        let cli: Cli = Cli::parse();
        let config: Config = match cli.config {
            Some(path) => path.into(),
            None => Config::read(),
        };
        Context {
            config: config,
            name: None,
            wordfile: cli.wordfile,
            wordlist: vec![],
            points: 0.0,
            speed: 0,
            last_tick: SystemTime::now(),
            chars: vec![],
            failed: 0,
            state: State::Run,
            last_state: State::Run,
            width: 0,
            height: 0,
        }
    }
}

impl Context {
    // add and remove character from the buffer
    // this is called when the user sends a key event
    pub fn enter_char(&mut self, new_char: char) {
        self.chars.push(new_char);
    }
    pub fn delete_char(&mut self) {
        self.chars.pop();
    }

    // create a word from the characters entered
    pub fn getinput(&self) -> String {
        self.chars.iter().cloned().collect::<String>()
    }

    // check if the word entered matches an answer
    // of one of the question in our list
    pub fn checkword(&mut self) {
        let word = self.getinput();
        if let Some(index) = self.wordlist.iter().position(|x| x.answer.contains(&word)) {
            self.wordlist.remove(index);
            self.points += self.calculatepoints(word);
            self.chars.clear();
        }
    }

    pub fn calculatepoints(&self, word: String) -> f32 {
        word.chars().count() as f32 / self.width as f32
    }

    pub fn renew(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.speed = self.config.startspeed + self.points as u64;

        let mut rng = rand::thread_rng();

        // count the words that are over the edge
        self.failed += self
            .wordlist
            .iter()
            .filter(|word| word.x > 0)
            .filter(|word| word.x as u16 >= width)
            .count();
        // retain only the words that are not over the edge
        self.wordlist
            .retain(|word| (((word.x > 0) && ((word.x as u16) < width)) || word.x <= 0));
        // react to resize: if a words y value is outside of the screen
        // (higher than height), simply generate a new y value
        self.wordlist
            .iter_mut()
            .filter(|word| word.y as u16 > height)
            .for_each(|word| word.y = rng.gen_range(1..height as i16));

        // based on the configured speed, we move all words one
        // position to the left and update the last_tick
        if self.last_tick + self.timeout() < SystemTime::now() {
            self.wordlist.iter_mut().for_each(|word| word.x += 1);
            self.last_tick = SystemTime::now();
        }

        // if the size of the wordlist is less then the configured minimum, we add words in anycase
        if self.wordlist.len() < self.config.minwords {
            self.addword();
        }
        // if the size of the wordlist is in our range, we roll the dice
        let range = self.config.minwords..=self.config.maxwords;
        if range.contains(&self.wordlist.len()) {
            self.maybeaddword()
        }
    }

    pub fn gameover(&self) -> bool {
        self.failed >= self.config.misses
    }

    pub fn addword(&mut self) {
        if let Some(wordfile) = &self.wordfile {
            if let Some(word) = Word::new(wordfile.to_path_buf(), self.width, self.height) {
                self.wordlist.push(word);
            }
        }
    }

    pub fn maybeaddword(&mut self) {
        let mut rng = rand::thread_rng();
        let minx = match self
            .wordlist
            .iter()
            .filter(|word| word.x > 0)
            .min_by_key(|word| word.x)
        {
            Some(word) => word.x,
            _ => 0,
        };
        let mut denom = 200 - minx as u32 - self.points as u32;
        if denom < 2 {
            denom = 2;
        }
        if rng.gen_ratio(1, denom) {
            self.addword();
        }
    }

    pub fn timeout(&self) -> Duration {
        match self.speed {
            0..=500 => Duration::from_millis(500 - self.speed),
            _ => Duration::from_millis(0),
        }
    }

    pub fn reset(&mut self) {
        self.writescores();
        self.wordfile = None;
        self.wordlist.clear();
        self.points = 0.0;
        self.failed = 0;
        self.chars.clear();
    }

    pub fn writescores(&self) {
        if self.points > 0.0 && self.wordfile.is_some() {
            let mut scores = Scores::read();
            scores.scores.push(self.into());
            scores.write(self.config.maxscores);
        }
    }
}

pub struct Word {
    pub question: String,
    pub answer: Vec<String>,
    pub x: i16,
    pub y: i16,
}

impl Word {
    pub fn new(wordfile: PathBuf, _width: u16, height: u16) -> Option<Word> {
        let rline = wordfile.randomline().ok()?;
        let mut iter = csv::ReaderBuilder::new()
            .flexible(true)
            .has_headers(false)
            .from_reader(rline.as_bytes())
            .into_records();
        let result = iter.next()?.ok()?;

        let items: Vec<String> = result.iter().map(|x| x.to_string()).collect();
        let (first, elements) = items.split_first()?;
        let question = first.to_string();
        let answer: Vec<String> = if elements.is_empty() {
            vec![question.clone()]
        } else {
            elements.iter().map(|x| x.to_string()).collect()
        };

        let len = question.len() as f32;
        Some(Word {
            question,
            answer,
            x: 0 - len as i16,
            y: rand::thread_rng().gen_range(1..height as i16),
        })
    }
}
