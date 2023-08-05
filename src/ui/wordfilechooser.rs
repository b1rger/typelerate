// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use super::components::centered_rect;
use super::FileChooser;
use crate::common::FileExtensions;
use ratatui::{prelude::*, widgets::*};

pub fn wordfilechooser<B: Backend>(f: &mut Frame<B>, files: &mut FileChooser) {
    let items: Vec<ListItem> = files
        .items
        .iter()
        .filter_map(|i| i.firstline().ok())
        .map(|i| ListItem::new(i))
        .collect();
    let area = centered_rect(40, 80, f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title("  Choose dictionary type  📚  ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double)
        .padding(Padding::uniform(1));
    let block_inner = block.inner(area);
    let [top, middle, bottom] = *Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)].as_ref()).split(block_inner) else { return; };
    let list = List::new(items).highlight_style(Style::default().bg(Color::Green));
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new("Choose a dictionary to play\n(use arrow keys to scroll, enter to select):")
            .alignment(Alignment::Center),
        top,
    );
    f.render_stateful_widget(list, middle, &mut files.state);
    f.render_widget(
        Paragraph::new("Keybindings:\n<ctrl>+q   quit\n<ctrl>+p   pause")
            .alignment(Alignment::Center),
        bottom,
    );
}
