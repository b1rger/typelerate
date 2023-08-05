// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::common::FileExtensions;
use ratatui::widgets::*;
use std::path::PathBuf;

pub struct FileChooser {
    pub items: Vec<PathBuf>,
    pub state: ListState,
}

impl FileChooser {
    pub fn new(paths: &Vec<PathBuf>) -> FileChooser {
        let mut items = vec![];
        for path in paths {
            if let Ok(entries) = path.read_dir() {
                let mut tmpvec = entries
                    .flat_map(|x| x)
                    .map(|entry| entry.path())
                    .filter(|x| x.valid())
                    .collect();
                items.append(&mut tmpvec);
            }
        }
        let mut state = ListState::default();
        state.select(Some(0));
        FileChooser {
            items: items,
            state: state,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&mut self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => 0,
        }
    }
}
