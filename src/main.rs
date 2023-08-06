// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use std::io::Stdout;

use anyhow::{Context, Result};
use ratatui::prelude::*;

mod cli;
mod common;
mod config;
mod context;
mod ui;
mod scores;

use crate::scores::Scores;

fn main() -> Result<()> {
    let mut ctx: context::Context = context::Context::default();
    let mut terminal = ui::setup().context("setup failed")?;
    run(&mut terminal, &mut ctx).context("app loop failed")?;
    ui::restore(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut ctx: &mut context::Context,
) -> Result<()> {
    let mut files = ui::FileChooser::new(&ctx.config.data);
    while ctx.state != context::State::Quit {
        terminal.draw(|f| ui(f, &mut ctx, &mut files))?;
        ui::handle_input(&mut ctx, &mut files)?;
    }
    ctx.writescores();
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, ctx: &mut context::Context, files: &mut ui::FileChooser) {
    match ctx.state {
        context::State::Score => {
            let scores: String = Scores::read().into();
            ui::popup(f, Some("Scores"), Some(scores.as_str()), None)
        },
        context::State::Pause => ui::popup(f, Some("Pause"), Some("Taking a break\n😴"), None),
        context::State::Run => ui::gamewrapper(f, ctx, files),
        context::State::Quit => return,
    }
}
