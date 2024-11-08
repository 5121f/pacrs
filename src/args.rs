use clap::Parser;

#[derive(Parser)]
pub enum Args {
    /// Print list of installed packages
    #[clap(short_flag = 'l')]
    List {
        /// Print list of packages that were updated in the repo
        /// (This does not affect the local index)
        #[clap(long, short, conflicts_with = "orphaned")]
        upgradable: bool,
        /// Print list of orphaned packages
        /// (packages which not installed explicitly and on which no package depends)
        #[clap(long, short, conflicts_with = "upgradable")]
        orphaned: bool,
    },
    /// Install packages
    #[clap(short_flag = 'i')]
    Install { packages: Vec<String> },
    /// Remove packages and unneeded dependencies
    #[clap(short_flag = 'r')]
    Remove {
        packages: Vec<String>,
        /// Remove orphaned packages
        /// (packages which not installed explicitly and on which no package depends)
        /// For only print this packages use `pacrs list --ororphaned`
        #[clap(long, short)]
        orphaned: bool,
    },
    /// Upgrade the system.
    #[clap(short_flag = 'u')]
    Upgrade {
        /// Packages to install with upgrade
        packages: Vec<String>,
    },
    #[clap(short_flag = 'n')]
    Info { package: String },
    #[clap(short_flag = 's')]
    Search { package: String },
    /// Mark packages
    #[clap(short_flag = 'm')]
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
    /// Mark packages as dependencie. For this operation, packages will have to be reinstalled
    #[clap(long, short)]
    pub dependencie: bool,
}
