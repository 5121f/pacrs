use clap::Parser;

#[derive(Parser)]
pub enum Args {
    List,
    Install { packages: Vec<String> },
    Upgrade { packages: Vec<String> },
    CheckForUpdates,
}
