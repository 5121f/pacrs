use clap::Parser;

#[derive(Parser)]
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
        #[clap(flatten)]
        remove_target: RemoveTarget,
        /// Automatically remove dependencies which become unneeded after removal of requested packages.
        #[clap(long, short = 'u')]
        clean_deps: bool,
    },
    /// Update installed packages with newer versions
    #[clap(visible_alias = "up")]
    Update {
        /// Packages to install with upgrade
        packages: Vec<String>,
        /// Don't show additional messages
        #[clap(long, short)]
        quiet: bool,
    },
    /// List all available packages
    #[clap(visible_alias = "pa")]
    Packages {
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
        /// Print list of packages which not finded in databases. In most cases it's AUR packages
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
    #[clap(visible_alias = "i")]
    Info {
        #[clap(required = true)]
        package: String,
    },
    /// List available updates
    #[clap(visible_alias = "lu")]
    ListUpdates,
    /// Print list of files
    /// (by default print all files)
    #[clap(visible_alias = "f")]
    Files {
        /// Print files related to specific package
        #[clap(conflicts_with = "find")]
        package: Option<String>,
        /// Find specific file among all packages
        #[clap(long, short, conflicts_with = "package")]
        find: Option<String>,
        /// Don't update files index
        #[clap(long, short = 'u')]
        not_update_index: bool,
        /// Don't show additional messages
        #[clap(long, short)]
        quiet: bool,
    },
    /// Cache
    #[clap(visible_alias = "cc")]
    Clean {
        /// Clean only cache of uninstalled packages
        #[clap(long, short)]
        uninstalled: bool,
    },
    /// Mark packages
    #[clap(visible_alias = "m")]
    Mark {
        #[clap(required = true)]
        packages: Vec<String>,
        #[clap(flatten)]
        mark_group: MarkGroup,
    },
}

#[derive(Debug, Parser)]
#[group(required = true)]
pub struct RemoveTarget {
    pub packages: Vec<String>,
    /// Remove orphaned packages
    /// (packages which not installed explicitly and on which no package depends)
    /// For only print this packages use `pacrs list --orphaned`
    #[clap(long, short)]
    pub orphaned: bool,
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
