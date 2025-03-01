// SPDX-License-Identifier: GPL-3.0-only

use crate::{cmds::pacman, command, pacman, pacrs::update_files_index};

fn package_files_global(
    name: &str,
    update_index: bool,
    quiet: bool,
) -> anyhow::Result<Vec<String>> {
    if update_index {
        update_files_index(quiet)?;
    }

    let lines = pacman()
        .arg("-Fl")
        .arg(name)
        .pipe_stderr()
        .execute_and_grub_lines()?;

    Ok(lines)
}

pub fn package_files(name: &str, update_index: bool, quiet: bool) -> anyhow::Result<()> {
    let files = pacman::files_of_installed_pkgs()
        .arg(name)
        .execute_and_grub_lines();

    let files = match files {
        Ok(files) => files,
        Err(command::Error::EndedWithNonZero {
            exit_status: _,
            command_name: _,
        }) => package_files_global(name, update_index, quiet)?,
        Err(err) => return Err(err.into()),
    };

    for line in files {
        println!("{line}");
    }

    Ok(())
}

pub fn find_file(file: &str, update_index: bool, quiet: bool) -> anyhow::Result<()> {
    if update_index {
        update_files_index(quiet)?;
    }
    pacman().arg("-F").arg(file).execute()?;
    Ok(())
}
