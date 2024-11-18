use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Context;
use fs_err::File;
use sysinfo::{ProcessesToUpdate, System};

use crate::cmds::pacman;

fn files_of_installed_pkgs() -> anyhow::Result<Vec<String>> {
    let lines = pacman().arg("-Ql").execute_and_grub_lines()?;
    let mut result = Vec::with_capacity(lines.len());
    for line in lines {
        let mut parts = line.split(' ');
        let file = parts.nth(1).context("Unable to parse pacman output")?;
        result.push(file.to_owned());
    }
    Ok(result)
}

/// Returns (process_name, file_name)
fn deleted_files_and_his_processes() -> anyhow::Result<Vec<(String, String)>> {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    let mut result = Vec::new();
    for (pid, process) in system.processes() {
        let path = Path::new("/proc").join(pid.to_string()).join("maps");
        let Ok(file) = File::open(path) else {
            continue;
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
            if parts[parts.len() - 1] != "(deleted)" {
                continue;
            }
            let fname = parts[parts.len() - 2];
            if fname.starts_with("/dev")
                || fname.starts_with("/run")
                || fname.starts_with("/drm")
                || fname.starts_with("/memfd")
                || fname.starts_with("/SYSV")
            {
                continue;
            }

            let process = process
                .exe()
                .map(|p| {
                    let file_name = p.file_name().unwrap_or_default().to_string_lossy();
                    file_name
                        .strip_suffix("(deleted)")
                        .map(ToString::to_string)
                        .unwrap_or(file_name.to_string())
                })
                .unwrap_or_else(|| process.name().to_string_lossy().to_string())
                .to_string();

            result.push((process, fname.to_owned()));
        }
    }
    Ok(result)
}

pub fn ps() -> anyhow::Result<()> {
    let pkgs_files = files_of_installed_pkgs()?;
    let mut processes = HashSet::new();
    for (process, file) in deleted_files_and_his_processes()? {
        if pkgs_files.contains(&file) {
            processes.insert(process.to_owned());
        }
    }
    for process in processes {
        println!("{process}");
    }
    Ok(())
}
