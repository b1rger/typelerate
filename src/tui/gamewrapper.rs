// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::common::FileExtensions;
use crate::context;
use ratatui::prelude::*;

pub fn gamewrapper<B: Backend>(
    f: &mut Frame<B>,
    ctx: &mut context::Context,
    files: &mut super::FileChooser,
) {
    if ctx.name.is_none() {
        super::namechooser(f, ctx);
    } else if ctx.wordfile.is_none() {
        if files
            .items
            .iter()
            .filter_map(|i| i.firstline().ok())
            .collect::<String>()
            .is_empty()
        {
            super::nodatafilepopup(f, ctx);
        } else {
            super::wordfilechooser(f, files);
            return;
        }
    } else {
        super::game(f, ctx);
    }
}
