use std::{io::Write, process::Command};

pub fn list() {
    let mut cmd = paru();
    cmd.arg("-Qq");
    execute(&mut cmd)
}

pub fn install(packages: Vec<String>) {
    let mut cmd = paru();
    cmd.arg("-S").args(packages);
    execute(&mut cmd)
}

fn paru() -> Command {
    Command::new("paru")
}

pub fn execute(cmd: &mut Command) {
    let output = cmd.output().unwrap();
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}
