// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context;
use crate::ui;
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

pub fn handle_input(ctx: &mut context::Context, files: &mut ui::FileChooser) -> Result<bool> {
    if event::poll(Duration::from_millis(10)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            if key.modifiers == KeyModifiers::CONTROL {
                if let KeyCode::Char('q') = key.code {
                    ctx.state = context::State::Quit
                }
                if let KeyCode::Char('l') = key.code {
                    ctx.chars.clear();
                }
                if let KeyCode::Char('p') = key.code {
                    ctx.state = match ctx.state {
                        context::State::Run => context::State::Pause,
                        _ => context::State::Run,
                    }
                }
                if let KeyCode::Char('r') = key.code {
                    ctx.reset();
                }
            } else {
                if let KeyCode::Char(to_insert) = key.code {
                    ctx.enter_char(to_insert);
                }
                if let KeyCode::Backspace = key.code {
                    ctx.delete_char();
                }
                if let KeyCode::Enter = key.code {
                    if ctx.wordfile.is_none() {
                        if !&files.items.is_empty() {
                            let pos = &files.selected();
                            let filename = &files.items[*pos];
                            ctx.wordfile = Some(filename.into());
                        }
                    }
                    if ctx.gameover() {
                        ctx.reset();
                    } else {
                        ctx.checkword();
                    }
                }
                if let KeyCode::Down = key.code {
                    files.next()
                }
                if let KeyCode::Up = key.code {
                    files.previous()
                }
            }
        }
    }
    Ok(false)
}
