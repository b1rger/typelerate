// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use ratatui::prelude::*;

pub fn help<B: Backend>(f: &mut Frame<B>) {
    let title = "   Help 🤝  ";
    super::popup(f, Some(title), Some("content"), None);
}
