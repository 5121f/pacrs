use clap::Parser;

#[derive(Parser)]
pub enum Args {
    /// Print list of installed packages
    List,
    /// Install packages
    Install {
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
    CheckForUpdates,
}
