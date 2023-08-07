// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::common::FileExtensions;
use std::path::PathBuf;
use ratatui::{prelude::*, widgets::*};

pub fn popup<B: Backend>(
    f: &mut Frame<B>,
    title: Option<&str>,
    message: Option<&str>,
    background: Option<ratatui::style::Color>,
) {
    let c = background.unwrap_or(ratatui::style::Color::DarkGray);
    let t = title.unwrap_or("No title");
    let m = message.unwrap_or("No message");

    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    let messageblock = Block::default()
        .title(t)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .padding(Padding::uniform(2))
        .bg(c);
    let messageblock_inner = messageblock.inner(area);
    f.render_widget(messageblock, area);
    f.render_widget(
        Paragraph::new(m)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        messageblock_inner,
    );
}

pub fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let width = width.min(r.width);
    let height = height.min(r.height);

    let sidewidth = (r.width - width) / 2;
    let sideheight = (r.height - height) / 2;

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(sideheight),
                Constraint::Length(height),
                Constraint::Length(sideheight),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(sidewidth),
                Constraint::Length(width),
                Constraint::Length(sidewidth),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

/*pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}*/

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
        state.select(None);
        FileChooser {
            items: items,
            state: state,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if (self.items.len() == 0) || (i >= self.items.len() - 1) {
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
                if self.items.len() == 0 {
                    0
                } else if i == 0 {
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
