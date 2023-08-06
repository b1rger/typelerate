// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context::Context;
use super::components::centered_rect;
use ratatui::{prelude::*, widgets::*};

pub fn namechooser<B: Backend>(f: &mut Frame<B>, ctx: &mut Context) {
    let area = centered_rect(40, 80, f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title("  Enter your name  ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double)
        .padding(Padding::uniform(1));
    let block_inner = block.inner(area);
    let [top, bottom] = *Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref()).split(block_inner) else { return; };
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new("Enter your name:")
            .alignment(Alignment::Center),
        top,
    );
    f.render_widget(
        Paragraph::new(ctx.getword())
            .alignment(Alignment::Center),
        bottom,
    );
}
