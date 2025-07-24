// SPDX-License-Identifier: GPL-3.0-only

use std::{env, io::Write, path::Path};

use clap::{Command, CommandFactory};
use clap_complete::Shell;
use fs_err::File;

use crate::args::Args;

const SHELLS: [(Shell, &str); 2] = [(Shell::Bash, "bash"), (Shell::Zsh, "zsh")];

pub fn generate() {
    let mut cmd = Args::command();
    for (shell, name) in SHELLS {
        let bash = generate_(shell, &mut cmd);
        let path = Path::new("completions").join(name);
        let mut file = File::create(path).unwrap();
        file.write_all(&bash).unwrap();
    }
}

fn generate_(shell: Shell, command: &mut Command) -> Vec<u8> {
    let mut buf = Vec::new();
    clap_complete::generate(shell, command, env!("CARGO_PKG_NAME"), &mut buf);
    buf
}
