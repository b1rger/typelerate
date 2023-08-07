// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use std::io::Stdout;

use ratatui::prelude::*;

mod cli;
mod common;
mod config;
mod context;
mod scores;
mod tui;

use crate::scores::Scores;

fn main() -> std::io::Result<()> {
    let mut ctx: context::Context = context::Context::default();
    let mut terminal = tui::setup()?;
    run(&mut terminal, &mut ctx)?;
    tui::restore(&mut terminal)?;
    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut ctx: &mut context::Context,
) -> std::io::Result<()> {
    let mut files = tui::FileChooser::new(&ctx.config.data);
    while ctx.state != context::State::Quit {
        terminal.draw(|f| tui(f, &mut ctx, &mut files))?;
        tui::handle_input(&mut ctx, &mut files)?;
    }
    ctx.writescores();
    Ok(())
}

fn tui<B: Backend>(f: &mut Frame<B>, ctx: &mut context::Context, files: &mut tui::FileChooser) {
    match ctx.state {
        context::State::Help => tui::help(f),
        context::State::Score => {
            let scores: String = Scores::read().into();
            tui::popup(f, Some("Scores"), Some(scores.as_str()), None)
        }
        context::State::Pause => tui::popup(f, Some("Pause"), Some("Taking a break\n😴"), None),
        context::State::Run => tui::gamewrapper(f, ctx, files),
        context::State::Quit => return,
    }
}
