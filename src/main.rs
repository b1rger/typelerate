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

use common::FileExtensions;

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
    if ctx.name.is_none() {
        ui::namechooser(f, ctx);
        return;
    }
    // if there is no wordfile set via commandline or already chosen,
    // lets display a dialog to let the user pick one
    if ctx.wordfile.is_none() {
        if files
            .items
            .iter()
            .filter_map(|i| i.firstline().ok())
            .collect::<String>()
            .is_empty()
        {
            ui::nodatafilepopup(f, ctx);
        } else {
            ui::wordfilechooser(f, files);
        }
    } else {
        ui::game(f, ctx);
    }
}
