use clap::Parser;

#[derive(Parser)]
pub enum Args {
    /// Print list of installed packages
    List {
        /// Print list of packages that were updated in the repo
        /// (This does not affect the local index)
        #[clap(long, short)]
        updated: bool,
    },
    /// Install packages
    Install {
        packages: Vec<String>,
    },
    /// Remove packages
    Remove {
        packages: Vec<String>,
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
}
