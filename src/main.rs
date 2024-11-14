mod alpm;
mod args;
mod cmds;
mod command;
mod pacrs;
mod temp_db;
mod utils;

use crate::{alpm::PacrsAlpm, args::Args, command::Cmd};

use anyhow::bail;
use args::{MarkGroup, RemoveTarget};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::Packages { orphaned, aur } => list(orphaned, aur)?,
        Args::Install { packages } => pacrs::install(packages)?,
        Args::Remove {
            remove_target,
            clean_deps,
        } => remove(remove_target, clean_deps)?,
        Args::Update { packages, quiet } => update(packages, quiet),
        Args::Info { package } => pacrs::info(package)?,
        Args::Search { package } => pacrs::search(package)?,
        Args::ListUpdates => pacrs::list_updates()?,
        Args::Files {
            package,
            find: file,
            not_update_index,
            quiet,
        } => files(package, file, not_update_index, quiet)?,
        Args::Clean { uninstalled } => cache(uninstalled)?,
        Args::Mark {
            packages,
            mark_group:
                MarkGroup {
                    explicit,
                    dependencie,
                },
        } => mark(packages, explicit, dependencie)?,
    }
    Ok(())
}

fn update(packages: Vec<String>, quiet: bool) {
    let result = pacrs::update(packages);

    match result {
        Ok(()) if !quiet => eprintln!(
            "Remember: if update system was aborted or error ends, \
            you need to finish the update before installing packages"
        ),
        Err(error) => {
            eprintln!("{error}");
            eprintln!(
                "Warning: The update ended with an error. \
                You need to finish update before installing packages."
            );
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}

fn files(
    package: Option<String>,
    file: Option<String>,
    not_update_index: bool,
    quiet: bool,
) -> anyhow::Result<()> {
    let update_index = !not_update_index;
    if update_index {
        pacrs::update_files_index(quiet)?;
    }

    if let Some(package) = package {
        return pacrs::list_files_of_package(&package);
    }
    if let Some(file) = file {
        return pacrs::find_file(&file);
    }
    pacrs::list_of_all_files()
}

fn cache(uninstalled: bool) -> anyhow::Result<()> {
    if uninstalled {
        return pacrs::cache_clean_uninstalled();
    }
    pacrs::cache_clean()
}

fn remove(remove_target: RemoveTarget, clean_deps: bool) -> anyhow::Result<()> {
    if remove_target.orphaned {
        return pacrs::remvoe_orphaned_packages(clean_deps);
    }
    pacrs::remove(remove_target.packages, clean_deps)
}

fn list_filter(list: Vec<String>, packages: Vec<String>, changed: bool) -> Vec<String> {
    if !(changed || packages.is_empty()) {
        return packages;
    }
    list.into_iter()
        .filter(|line| packages.contains(line))
        .collect()
}

fn list(orphaned: bool, aur: bool) -> anyhow::Result<()> {
    let mut changed = false;
    let mut list = Vec::new();

    if orphaned {
        list = list_filter(list, pacrs::orphaned_packages()?, changed);
        changed = true;
    }
    if aur {
        list = list_filter(list, pacrs::list_aur()?, changed);
        changed = true;
    }

    if !changed {
        pacrs::list()?;
        return Ok(());
    }

    for package in list {
        println!("{package}");
    }

    Ok(())
}

fn mark(packages: Vec<String>, explicit: bool, dependencie: bool) -> anyhow::Result<()> {
    if explicit {
        return pacrs::mark_explicit(packages);
    }
    if dependencie {
        return pacrs::mark_dep(packages);
    }
    bail!("No one parameter specified");
}
