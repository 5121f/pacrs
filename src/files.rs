// SPDX-License-Identifier: GPL-3.0-only

use map_self::MapSelf;

use crate::{
    cmds::pacman,
    command, pacman,
    pacrs::{parse_pacman_files_output, update_files_index},
};

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

    parse_pacman_files_output(&lines)?.map_self(Ok)
}

pub fn packages_files_local() -> anyhow::Result<Vec<String>> {
    let lines = pacman::files_of_installed_pkgs().execute_and_grub_lines()?;
    parse_pacman_files_output(&lines)
}

pub fn package_files(name: &str, update_index: bool, quiet: bool) -> anyhow::Result<()> {
    let package_files = pacman::files_of_installed_pkgs()
        .arg(name)
        .execute_and_grub_lines();

    let lines = match package_files {
        Ok(lines) => parse_pacman_files_output(&lines)?,
        Err(command::Error::EndedWithNonZero {
            exit_status: _,
            command_name: _,
        }) => package_files_global(name, update_index, quiet)?,
        Err(err) => return Err(err.into()),
    };

    for line in lines {
        println!("{line}");
    }
    Ok(())
}

pub fn find_file(file: &str, update_index: bool, quiet: bool) -> anyhow::Result<()> {
    if update_index {
        update_files_index(quiet)?;
    }
    let pacman = pacman();
    pacman.arg("-F").arg(file).execute()?;
    Ok(())
}
