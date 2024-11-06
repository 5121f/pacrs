use std::{io::Write, process::Command};

pub struct Pacman {
    command: Command,
}
impl Pacman {
    pub fn new() -> Self {
        let command = Command::new("pacman");
        Self { command }
    }

    pub fn list(&mut self) -> &mut Self {
        self.command.arg("-Qq");
        self
    }

    pub fn run(&mut self) {
        let output = self.command.output().unwrap();
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
    }
}
