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

    pub fn sure(
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
        Ok(match buf.trim().to_lowercase().as_str() {
            "y" => Answer::Yes,
            "" => default_ansver,
            _ => Answer::No,
        })
    }
}

pub enum Answer {
    Yes,
    No,
}

impl Answer {
    pub fn is_no(&self) -> bool {
        matches!(self, Answer::No)
    }
}
