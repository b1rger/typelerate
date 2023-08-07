// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use super::components::centered_rect;
use super::FileChooser;
use crate::common::FileExtensions;
use ratatui::{prelude::*, widgets::*};

pub fn wordfilechooser<B: Backend>(f: &mut Frame<B>, files: &mut FileChooser) {
    let area = centered_rect(60, 40, f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title("  Choose dictionary  📚  ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double)
        .padding(Padding::uniform(2));
    let mut inner_area = block.inner(area);
    inner_area.y = (inner_area.y + 1).min(area.y + area.width);
    inner_area.height = inner_area.height - 1;

    let items: Vec<ListItem> = files
        .items
        .iter()
        .filter_map(|i| i.firstline().ok())
        .map(|i| ListItem::new(i))
        .collect();
    let list = List::new(items).highlight_style(Style::default().bg(Color::Green));

    f.render_widget(block, area);
    f.render_stateful_widget(list, inner_area, &mut files.state);
}
