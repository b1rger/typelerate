// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context::Context;
use super::components::centered_rect;
use ratatui::{prelude::*, widgets::*};

pub fn namechooser<B: Backend>(f: &mut Frame<B>, ctx: &mut Context) {
    let area = centered_rect(60, 40, f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title("  Enter your name  ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Double)
        .padding(Padding::uniform(2));
    let mut inner_area = block.inner(area);
    inner_area.height = 1;
    inner_area.y = (inner_area.y + 2).min(area.y+area.width);
    f.render_widget(block, area);
    f.render_widget(
        Paragraph::new(ctx.getinput())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Black).bg(Color::White)),
        inner_area,
    );
}
