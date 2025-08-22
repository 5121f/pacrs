// SPDX-License-Identifier: GPL-3.0-only

use std::fmt;
use std::io;
use std::io::{Stdin, Stdout, Write};

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
        let mut buf = self.read_single_line()?;
        buf.make_ascii_lowercase();
        Ok(match buf.trim() {
            "y" | "yes" => Answer::Yes,
            "" => default_ansver,
            _ => Answer::No,
        })
    }

    pub fn read_single_line(&mut self) -> io::Result<String> {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf)?;
        Ok(buf)
    }
}

pub enum Answer {
    Yes,
    No,
}

impl Answer {
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Yes => true,
            Self::No => false,
        }
    }
}
