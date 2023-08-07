// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use ratatui::prelude::*;

pub fn help<B: Backend>(f: &mut Frame<B>) {
    let title = "   Help 🤝  ";
    let help = "The goal of the game is to prevent the words flying across the screen from reaching the other side. You do that by either typing the words *or* typing an answer that hides behind the question flying across the screen (that depends on the dictionary).

        Keybindings:
        <ctrl>+l → clear input
        <ctrl>+h → help
        <ctrl>+p → pause
        <ctrl>+s → scoreboard
        <ctrl>+q → quit
        <ctrl>+r → reset";
    super::popup(f, Some(title), Some(help), None);
}
