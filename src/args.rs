use clap::Parser;

#[derive(Parser)]
pub enum Args {
    /// Install packages
    #[clap(visible_alias = "in")]
    Install {
        #[clap(required = true)]
        packages: Vec<String>,
    },
    /// Remove packages and unneeded dependencies
    #[clap(visible_alias = "rm")]
    Remove(#[clap(flatten)] RemoveGroup),
    /// Upgrade the system.
    #[clap(visible_alias = "up")]
    Upgrade {
        /// Packages to install with upgrade
        packages: Vec<String>,
    },
    /// Print list of installed packages
    #[clap(visible_alias = "pa")]
    Packages {
        /// Print list of packages that were updated in the repo
        /// (This does not affect the local index)
        #[clap(long, short, conflicts_with = "orphaned")]
        upgradable: bool,
        /// Print list of orphaned packages
        /// (packages which not installed explicitly and on which no package depends)
        #[clap(long, short, conflicts_with = "upgradable")]
        orphaned: bool,
    },
    /// Search packages
    #[clap(visible_alias = "se")]
    Search {
        #[clap(required = true)]
        package: String,
    },
    /// Print info about package
    #[clap(visible_alias = "i")]
    Info {
        #[clap(required = true)]
        package: String,
    },
    #[clap(visible_alias = "ca")]
    Cache {
        /// Print occupied size on disk by cache
        #[clap(long, short)]
        size: bool,
        #[clap(flatten)]
        clean: CacheCleanGroup,
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
#[group(requires_all = ["clean"])]
pub struct CacheCleanGroup {
    /// Clean packages from cache and unused repos. By default clean all packages from cache
    #[clap(long, short)]
    pub clean: bool,
    /// Clean only cache of uninstalled packages
    #[clap(long, short)]
    pub uninstalled: bool,
}

#[derive(Debug, Parser)]
#[group(required = true)]
pub struct RemoveGroup {
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
