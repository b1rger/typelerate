// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, author, about = "A typing game with guessing enhancement")]
pub struct Cli {
    #[arg(short, long)]
    pub wordfile: Option<PathBuf>,
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            wordfile: None,
            config: None,
        }
    }
}
