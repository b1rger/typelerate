// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context;
use ratatui::prelude::*;

pub fn nodatafilepopup<B: Backend>(f: &mut Frame<B>, ctx: &mut context::Context) {
    let mut filelist: Vec<String> = vec![];
    for path in &ctx.config.data {
        filelist.push(format!("{}", path.display()));
    }
    super::popup(f, Some("   No usable datafiles found 😞   "), Some(format!("\nI have looked in the following directories, but found no usable datafiles:\n\n{}\n\nType <ctrl>+q to quit.", filelist.join("\n")).as_str()), None);
}
