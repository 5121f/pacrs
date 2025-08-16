// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap};
use std::io::{BufRead, BufReader};

use fs_err::File;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind, Users};
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::args::PsSortBy;
use crate::pacman;
use crate::utils::{JoinError, is_root};

#[derive(PartialEq, Eq, Hash, Tabled)]
struct Process {
    pid: Pid,
    user_name: String,
    command: String,
}

impl Process {
    fn new(process: &sysinfo::Process, users: &Users) -> Self {
        Self {
            pid: process.pid(),
            user_name: process_owner(process, users),
            command: get_process_command(process).to_string(),
        }
    }
}

fn files_of_installed_pkgs() -> anyhow::Result<BTreeSet<String>> {
    let files = pacman::files_of_installed_pkgs().execute_and_grub_lines()?;
    // We assume that one file corresponds to one package
    let lines = BTreeSet::from_iter(files);
    Ok(lines)
}

fn get_process_command(process: &sysinfo::Process) -> Cow<'_, str> {
    process.exe().map_or_else(
        || process.name().to_string_lossy(),
        |p| {
            let file_name = p.file_name().unwrap_or_default().to_string_lossy();
            file_name
                .strip_suffix("(deleted)")
                .map(ToString::to_string)
                .map(Cow::Owned)
                .unwrap_or(file_name)
        },
    )
}

fn process_owner(process: &sysinfo::Process, users: &Users) -> String {
    fn inner(process: &sysinfo::Process, users: &Users) -> Option<String> {
        let uid = process.user_id()?;
        let user = users.get_user_by_id(uid)?;
        Some(user.name().to_owned())
    }

    inner(process, users).unwrap_or_else(|| String::from("Unknown"))
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

fn process_has_deleted_files(pid: Pid) -> anyhow::Result<BTreeSet<String>> {
    let mut result = BTreeSet::new();
    let path = format!("/proc/{pid}/maps");
    let file = match File::open(path) {
        Ok(value) => value,
        Err(err) => {
            log::error!("{err}");
            return Ok(result);
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_ascii_whitespace().skip(5);

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
            || fname.starts_with('[')
        {
            continue;
        }

        result.insert(fname.to_owned());
    }

    Ok(result)
}

fn deleted_files_and_his_processes() -> anyhow::Result<HashMap<Process, BTreeSet<String>>> {
    let system = configured_system();
    let users = Users::new_with_refreshed_list();

    let mut result = HashMap::new();
    for (pid, process) in system.processes() {
        let files = process_has_deleted_files(*pid)?;
        if !files.is_empty() {
            result.insert(Process::new(process, &users), files);
        }
    }
    Ok(result)
}

fn processes_with_deleted_files() -> anyhow::Result<Vec<Process>> {
    let pkgs_files = std::thread::spawn(files_of_installed_pkgs);
    let deleted_files_and_his_processes = std::thread::spawn(deleted_files_and_his_processes);

    let pkgs_files = pkgs_files.join_err_map()??;
    let deleted_files_and_his_processes = deleted_files_and_his_processes.join_err_map()??;

    Ok(deleted_files_and_his_processes
        .into_iter()
        .filter_map(|(process, files)| {
            files
                .iter()
                .any(|f| pkgs_files.contains(f))
                .then_some(process)
        })
        .collect())
}

pub fn ps(
    sort_by: Option<PsSortBy>,
    shorter: bool,
    reverse: bool,
    quiet: bool,
) -> anyhow::Result<()> {
    if !quiet && !is_root() {
        eprintln!(
            "Not running as root you are limited to searching for files you have permission. \
            The result might be incomplete.\n"
        );
    }

    let processes = processes_with_deleted_files()?;

    if processes.is_empty() {
        if !quiet {
            println!("The processes using removed files were not found.");
        }
        return Ok(());
    }

    if shorter {
        short_print(processes, reverse);
    } else {
        long_print(processes, reverse, sort_by);
    }

    Ok(())
}

fn short_print(processes: Vec<Process>, reverse: bool) {
    let mut command_names: Vec<String> = processes.into_iter().map(|p| p.command).collect();

    command_names.sort();
    command_names.dedup();

    if reverse {
        command_names.reverse();
    }

    for command in command_names {
        println!("{command}");
    }
}

fn long_print(mut processes: Vec<Process>, reverse: bool, sort_by: Option<PsSortBy>) {
    match sort_by {
        None => {}
        Some(PsSortBy::Pid) => processes.sort_by(|a, b| a.pid.cmp(&b.pid)),
        Some(PsSortBy::User) => processes.sort_by(|a, b| a.user_name.cmp(&b.user_name)),
        Some(PsSortBy::Command) => processes.sort_by(|a, b| a.command.cmp(&b.command)),
    }

    if reverse {
        processes.reverse();
    }

    let table = Table::new(&processes).with(Style::psql()).to_string();
    println!("{table}");
}
