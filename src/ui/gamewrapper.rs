// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context;
use crate::ui;
use crate::common::FileExtensions;
use ratatui::prelude::*;

pub fn gamewrapper<B: Backend>(f: &mut Frame<B>, ctx: &mut context::Context, files: &mut ui::FileChooser) {
    if ctx.name.is_none() {
        ui::namechooser(f, ctx);
    } else if ctx.wordfile.is_none() {
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
        return;
        }
    } else {
        ui::game(f, ctx);
    }
}
