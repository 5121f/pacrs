// SPDX-License-Identifier: GPL-3.0-only

use std::fmt;
use std::io;
use std::io::{Stdin, Stdout, Write};
use std::ops::Deref;

pub struct Cli {
    stdout: Stdout,
    stdin: Stdin,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            stdout: io::stdout(),
            stdin: io::stdin(),
        }
    }

    pub fn confirm(
        &mut self,
        question: impl fmt::Display,
        default_ansver: Answer,
    ) -> io::Result<Answer> {
        print!("{question} ");
        match default_ansver {
            Answer::Yes => print!("[Y/n] "),
            Answer::No => print!("[y/N] "),
        }
        self.stdout.flush()?;
        let mut buf = String::new();
        self.stdin.read_line(&mut buf)?;
        Ok(match buf.to_lowercase().trim() {
            "y" | "yes" => Answer::Yes,
            "" => default_ansver,
            _ => Answer::No,
        })
    }
}

pub enum Answer {
    Yes,
    No,
}

impl Deref for Answer {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Yes => &true,
            Self::No => &false,
        }
    }
}
