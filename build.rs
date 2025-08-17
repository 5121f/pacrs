use std::{fs::File, io::Write, path::Path};

use clap::{Command, CommandFactory};
use clap_complete::{Generator, Shell};

include!("src/args.rs");

const SHELLS: [(Shell, &str); 2] = [(Shell::Bash, "bash"), (Shell::Zsh, "zsh")];

fn main() {
    let mut args = Args::command();
    for (shell, shell_name) in SHELLS {
        let content = generate(&mut args, shell);
        let path = Path::new("completions").join(shell_name);
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(&content).expect("Failed to write in file");
    }
}

fn generate(cmd: &mut Command, generator: impl Generator) -> Vec<u8> {
    let mut buf = Vec::new();
    clap_complete::generate(generator, cmd, "pacrs", &mut buf);
    buf
}
