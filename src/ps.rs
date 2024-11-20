use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Context;
use fs_err::File;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind, Users};
use tabled::{settings::Style, Table, Tabled};

use crate::pacman;

#[derive(PartialEq, Eq, Hash, Tabled)]
struct Process {
    pid: Pid,
    user_name: String,
    command: String,
}

impl Process {
    fn new(process: &sysinfo::Process, users: &Users) -> Self {
        let pid = process.pid();
        let command = get_process_command(process);
        let user_name =
            user_name_by_process(process, users).unwrap_or_else(|| String::from("Unknown"));

        Self {
            pid,
            user_name,
            command,
        }
    }
}

fn files_of_installed_pkgs() -> anyhow::Result<HashSet<String>> {
    let lines = pacman::files_of_installed_pkgs().execute_and_grub_lines()?;
    // We assume that one file corresponds to one package
    let mut result = HashSet::with_capacity(lines.len());
    for line in lines {
        let mut parts = line.split(' ');
        let file = parts.nth(1).context("Unable to parse pacman output")?;
        result.insert(file.to_owned());
    }
    Ok(result)
}

fn get_process_command(process: &sysinfo::Process) -> String {
    process
        .exe()
        .map(|p| {
            let file_name = p.file_name().unwrap_or_default().to_string_lossy();
            file_name
                .strip_suffix("(deleted)")
                .map(ToString::to_string)
                .unwrap_or(file_name.to_string())
        })
        .unwrap_or_else(|| process.name().to_string_lossy().to_string())
        .to_string()
}

fn user_name_by_process(process: &sysinfo::Process, users: &Users) -> Option<String> {
    let uid = process.user_id()?;
    let user = users.get_user_by_id(uid)?;
    Some(user.name().to_owned())
}

fn configured_system() -> System {
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        false,
        ProcessRefreshKind::new()
            .with_user(UpdateKind::OnlyIfNotSet)
            .with_exe(UpdateKind::OnlyIfNotSet),
    );
    system
}

async fn deleted_files_and_his_processes() -> anyhow::Result<Vec<(Process, String)>> {
    let system_handler = tokio::spawn(async { configured_system() });
    let users_handler = tokio::spawn(async { Users::new_with_refreshed_list() });

    let system = system_handler.await?;
    let users = users_handler.await?;

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

            result.push((Process::new(process, &users), fname.to_owned()));
        }
    }
    Ok(result)
}

pub async fn ps() -> anyhow::Result<()> {
    let pkgs_files_handler = tokio::spawn(async { files_of_installed_pkgs() });
    let deleted_files_and_his_processes_handler = tokio::spawn(deleted_files_and_his_processes());

    let pkgs_files = pkgs_files_handler.await??;
    let deleted_files_and_his_processes = deleted_files_and_his_processes_handler.await??;

    let mut processes = HashSet::new();
    for (process, file) in deleted_files_and_his_processes {
        if pkgs_files.contains(&file) {
            processes.insert(process);
        }
    }

    if processes.is_empty() {
        return Ok(());
    }

    let table = Table::new(&processes).with(Style::psql()).to_string();
    println!("{table}");

    Ok(())
}
