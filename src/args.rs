// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;

#[derive(Parser)]
#[command(version = clap::crate_version!(), about = clap::crate_description!())]
pub enum Args {
    /// Install packages
    #[clap(visible_alias = "in")]
    Install {
        #[clap(required = true)]
        packages: Vec<String>,
    },
    /// Remove (uninstall) packages
    #[clap(visible_alias = "rm")]
    Remove {
        #[clap(required = true)]
        packages: Vec<String>,
        /// Automatically remove dependencies which become unneeded after removal of requested packages.
        #[clap(long, short, short = 'u')]
        clean_deps: bool,
    },
    /// Remove unneeded packages
    #[clap(visible_alias = "ar")]
    Autoremove,
    /// Update installed packages with newer versions
    #[clap(visible_alias = "up")]
    Update {
        /// Packages to install with update
        packages: Vec<String>,
        /// Show less inforamation
        #[clap(long, short)]
        quiet: bool,
    },
    /// List all available packages
    #[clap(visible_alias = "pa")]
    Packages {
        /// Search for given string in installed packages
        #[clap(long, short, value_name = "REGEX", conflicts_with_all = ["explicit", "deps", "orphaned", "aur"])]
        search: Option<String>,
        /// List of explicit installed packages
        #[clap(long, short)]
        explicit: bool,
        /// List of packages installed as dependencie
        #[clap(long, short)]
        deps: bool,
        /// List of orphaned packages
        /// (packages which not installed explicitly and on which no package depends)
        #[clap(long, short)]
        orphaned: bool,
        /// Print list of packages which not found in databases. In most cases it's AUR packages
        #[clap(long, short)]
        aur: bool,
    },
    /// Search for packages matching any of the given search strings.
    #[clap(visible_alias = "se")]
    Search {
        #[clap(required = true)]
        package: String,
    },
    /// Displays detailed information about the specified packages
    #[clap(visible_alias = "if")]
    Info {
        #[clap(required = true)]
        package: String,
    },
    /// List available updates
    #[clap(visible_alias = "lu")]
    ListUpdates,
    /// Print list of files
    /// (by default print all files)
    #[clap(visible_alias = "fl")]
    Files {
        /// Print files related to specific package
        #[clap(conflicts_with = "find")]
        package: Option<String>,
        /// Find specific file among all packages
        #[clap(long, short, conflicts_with = "package")]
        find: Option<String>,
        /// Don't update files index
        #[clap(long, short = 'U')]
        not_update_index: bool,
        /// Show less inforamation
        #[clap(long, short)]
        quiet: bool,
    },
    /// Cache
    #[clap(visible_alias = "cc")]
    Clean {
        /// Clean cache of uninstalled packages
        #[clap(long, short, conflicts_with = "aur")]
        uninstalled: bool,
        /// Clean AUR cache
        #[clap(long, short, conflicts_with = "uninstalled")]
        aur: bool,
    },
    /// Mark packages
    #[clap(visible_alias = "mr")]
    Mark {
        #[clap(required = true)]
        packages: Vec<String>,
        #[clap(flatten)]
        mark_group: MarkGroup,
    },
    /// Processes which continue to use deleted files.
    /// This command is actual after updating the system and they can indicate the processes that
    /// should be restarted.
    Ps {
        /// Sort output
        #[clap(long, short, value_parser = ["pid", "user", "command"], conflicts_with = "shorter")]
        sort_by: Option<String>,
        /// Show only list of commands instead of table
        #[clap(long, short = 'o', conflicts_with = "sort_by")]
        shorter: bool,
        /// Reverse sorting
        #[clap(long, short)]
        reverse: bool,
        /// Don't show additional messages
        #[clap(long, short)]
        quiet: bool,
    },
}

#[derive(Debug, Parser)]
#[group(required = true)]
pub struct MarkGroup {
    /// Mark packages as installed explicit
    #[clap(long, short)]
    pub explicit: bool,
    /// Mark packages as dependencie (non-explicit installed)
    #[clap(long, short)]
    pub dependencie: bool,
}
