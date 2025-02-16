// SPDX-License-Identifier: GPL-3.0-only

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::{BufRead, BufReader},
};

use anyhow::bail;
use fs_err::File;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind, Users};
use tabled::{settings::Style, Table, Tabled};
use tokio::join;

use crate::{files::packages_files_local, utils::is_root};

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

fn files_of_installed_pkgs() -> anyhow::Result<BTreeSet<String>> {
    let files = packages_files_local()?;
    // We assume that one file corresponds to one package
    let lines = BTreeSet::from_iter(files);
    Ok(lines)
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
        ProcessRefreshKind::nothing()
            .with_user(UpdateKind::OnlyIfNotSet)
            .with_exe(UpdateKind::OnlyIfNotSet),
    );
    system
}

fn process_has_deleted_files(pid: &Pid) -> anyhow::Result<HashSet<String>> {
    let mut result = HashSet::new();
    let path = format!("/proc/{pid}/maps");
    let Ok(file) = File::open(path) else {
        return Ok(result);
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(' ').skip(5).filter(|s| !s.is_empty());

        let Some(fname) = parts.next() else {
            continue;
        };

        let deleted = parts.next().is_some_and(|part| part == "(deleted)");
        if !deleted {
            continue;
        }

        if fname.starts_with("/dev")
            || fname.starts_with("/run")
            || fname.starts_with("/drm")
            || fname.starts_with("/memfd")
            || fname.starts_with("/SYSV")
            || fname.starts_with("[")
        {
            continue;
        }

        result.insert(fname.to_owned());
    }

    Ok(result)
}

async fn deleted_files_and_his_processes() -> anyhow::Result<HashMap<Process, HashSet<String>>> {
    let (system, users) = join!(
        tokio::spawn(async { configured_system() }),
        tokio::spawn(async { Users::new_with_refreshed_list() })
    );
    let (system, users) = (system?, users?);

    let mut result = HashMap::new();
    for (pid, process) in system.processes() {
        let files = process_has_deleted_files(pid)?;
        if !files.is_empty() {
            result.insert(Process::new(process, &users), files);
        }
    }
    Ok(result)
}

pub async fn ps(
    sort_by: Option<String>,
    shorter: bool,
    reverse: bool,
    quiet: bool,
) -> anyhow::Result<()> {
    if !quiet && !is_root() {
        eprintln!("Running without root privileges. Not all processes can be displayed.\n")
    }

    let (pkgs_files, deleted_files_and_his_processes) = join!(
        tokio::spawn(async { files_of_installed_pkgs() }),
        tokio::spawn(deleted_files_and_his_processes())
    );

    let pkgs_files = pkgs_files??;
    let deleted_files_and_his_processes = deleted_files_and_his_processes??;

    let mut processes = Vec::new();
    for (process, files) in deleted_files_and_his_processes {
        if files.iter().any(|f| pkgs_files.contains(f)) {
            processes.push(process);
        }
    }

    if processes.is_empty() {
        return Ok(());
    }

    if shorter {
        let mut command_names: Vec<String> = processes.into_iter().map(|p| p.command).collect();
        command_names.sort();
        command_names.dedup();
        for command in command_names {
            println!("{command}");
        }
        return Ok(());
    }

    if let Some(sort_by) = sort_by {
        match sort_by.as_str() {
            "pid" => processes.sort_by(|a, b| a.pid.cmp(&b.pid)),
            "user" => processes.sort_by(|a, b| a.user_name.cmp(&b.user_name)),
            "command" => processes.sort_by(|a, b| a.command.cmp(&b.command)),
            _ => bail!("Wrong sort-by value"),
        }
    }

    if reverse {
        processes.reverse();
    }

    let table = Table::new(&processes).with(Style::psql()).to_string();
    println!("{table}");

    Ok(())
}
