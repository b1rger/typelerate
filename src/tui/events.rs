// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

pub fn handle_input(
    ctx: &mut context::Context,
    files: &mut super::FileChooser,
) -> std::io::Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if key.modifiers == KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('q') => ctx.state = context::State::Quit,
                    KeyCode::Char('l') => ctx.chars.clear(),
                    KeyCode::Char('p') => {
                        ctx.state = match ctx.state {
                            context::State::Pause => context::State::Run,
                            _ => context::State::Pause,
                        }
                    }
                    KeyCode::Char('s') => {
                        ctx.state = match ctx.state {
                            context::State::Score => context::State::Run,
                            _ => context::State::Score,
                        }
                    }
                    KeyCode::Char('h') => {
                        ctx.state = match ctx.state {
                            context::State::Help => context::State::Run,
                            _ => context::State::Help,
                        }
                    }
                    KeyCode::Char('r') => ctx.reset(),
                    _ => (),
                }
            } else {
                match key.code {
                    KeyCode::Down => files.next(),
                    KeyCode::Up => files.previous(),
                    KeyCode::Backspace => ctx.delete_char(),
                    KeyCode::Char(input) => ctx.enter_char(input),
                    KeyCode::Enter => {
                        if ctx.name.is_none() {
                            ctx.name = Some(ctx.getinput());
                            ctx.chars.clear();
                            return Ok(true);
                        }
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
                    _ => (),
                }
            }
        }
    }
    Ok(false)
}
