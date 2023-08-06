// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::context;
use crate::ui;
use crate::common::FileExtensions;
use ratatui::{prelude::*, widgets::*};

pub fn game<B: Backend>(f: &mut Frame<B>, ctx: &mut context::Context) {
    let outer = f.size();
    let [top, bottom] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(outer.height - 3), Constraint::Length(3)].as_ref())
        .split(outer)
    else {
        return;
    };

    /* top area */

    let mut wordfilename = String::new();
    if let Some(wordfile) = &ctx.wordfile {
        wordfilename = wordfile.firstline().unwrap_or_default();
    }

    let top_block = Block::default()
        .borders(Borders::ALL)
        .title(
            block::Title::from(format!(
                "   {} | {} | speed {}   ",
                env!("CARGO_CRATE_NAME"), wordfilename, ctx.speed
            ))
            .alignment(Alignment::Center),
        )
        .border_type(BorderType::Rounded);

    let top_inner = top_block.inner(top);
    f.render_widget(top_block, top);
    if ctx.gameover() {
        ui::popup(
            f,
            Some("Game Over"),
            Some("Game over!\n😱\nType enter to play again"),
            None,
        );
    } else {
        if ctx.state == context::State::Pause {
            ui::popup(f, Some("Pause"), Some("Taking a break\n😴"), None);
        } else {
            ctx.renew(top_inner.width, top_inner.height);
            // offset:
            // first is offset of the y axis from above
            // second is offset of the x axis from the right

            for word in &mut ctx.wordlist {
                let string = word.question.as_str();
                let y = word.y as u16;
                if word.x.is_negative() {
                    let area = Rect::new(1, y, top_inner.width - 1, top_inner.height - y);
                    f.render_widget(
                        Paragraph::new(string.white()).scroll((0, word.x.abs() as u16)),
                        area,
                    );
                } else {
                    let third = top_inner.width / 3;
                    let first = 0..third;
                    let second = third..third * 2;
                    let x = word.x.abs() as u16;

                    let color = match x {
                        x if first.contains(&x) => Color::Green,
                        x if second.contains(&x) => Color::Yellow,
                        _ => Color::Red,
                    };

                    let area = Rect::new(x + 1, y, top_inner.width - x, top_inner.height - y);
                    f.render_widget(Paragraph::new(string.fg(color)), area);
                }
            }
        }
    }

    /* bottom area */

    let [bottom_left, bottom_right] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(bottom.width - 20), Constraint::Length(20)].as_ref())
        .split(bottom)
    else {
        return;
    };

    let input_block = Block::default().borders(Borders::ALL);
    let input_area = input_block.inner(bottom_left);
    f.render_widget(Paragraph::new(ctx.getword()), input_area);
    f.render_widget(input_block, bottom_left);

    let status_block = Block::default().borders(Borders::ALL);
    let status_area = status_block.inner(bottom_right);
    f.render_widget(
        Paragraph::new(format!("{}|{:.3}", ctx.failed, ctx.points)).alignment(Alignment::Right),
        status_area,
    );
    f.render_widget(status_block, bottom_right);
}
