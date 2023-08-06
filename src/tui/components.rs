// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use ratatui::{prelude::*, widgets::*};

pub fn popup<B: Backend>(
    f: &mut Frame<B>,
    title: Option<&str>,
    message: Option<&str>,
    background: Option<ratatui::style::Color>,
) {
    let c = background.unwrap_or(ratatui::style::Color::DarkGray);
    let t = title.unwrap_or("No title");
    let m = message.unwrap_or("No message");

    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    let messageblock = Block::default()
        .title(t)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .bg(c);
    let messageblock_inner = messageblock.inner(area);
    f.render_widget(messageblock, area);
    f.render_widget(
        Paragraph::new(m)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        messageblock_inner,
    );
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
