use clap::Parser;

#[derive(Parser)]
pub enum Args {
    /// Print list of installed packages
    List {
        /// Print list of packages that were updated in the repo
        /// (This does not affect the local index)
        #[clap(long, short)]
        upgradable: bool,
        /// Print list of orphaned packages
        /// (packages which not installed explicitly and on which no package depends)
        #[clap(long, short)]
        orphaned: bool,
    },
    /// Install packages
    Install {
        packages: Vec<String>,
    },
    /// Remove packages and unneeded dependencies
    Remove {
        packages: Vec<String>,
        /// Remove orphaned packages
        /// (packages which not installed explicitly and on which no package depends)
        /// For only print this packages use `pacrs list --ororphaned`
        #[clap(long, short)]
        orphaned: bool,
    },
    /// Upgrade the system.
    Upgrade {
        /// Packages to install with upgrade
        packages: Vec<String>,
    },
    Info {
        package: String,
    },
    Search {
        package: String,
    },
    /// Mark packages
    Mark {
        #[clap(required = true)]
        packages: Vec<String>,
        #[clap(flatten)]
        mark_group: MarkGroup,
    },
}

#[derive(Debug, Parser)]
#[group(required = true)]
pub struct MarkGroup {
    /// Mark packages as installed explicit
    #[clap(long, short)]
    pub explicit: bool,
}
