use std::{fs, io::Write, path::Path, process::Command};

pub fn list() {
    let mut cmd = paru();
    cmd.arg("-Qq");
    execute(&mut cmd)
}

pub fn install(packages: Vec<String>) {
    let mut cmd = paru();
    cmd.arg("-S").args(packages);
    execute(&mut cmd)
}

pub fn check_for_updates() {
    let temp_db_dir = "/tmp/pacrs/db";
    fs::create_dir_all(temp_db_dir).unwrap();
    let conf = pacmanconf::Config::new().unwrap();
    let temp_local_db = Path::new(temp_db_dir).join("local");
    if !temp_local_db.exists() {
        std::os::unix::fs::symlink(Path::new(&conf.db_path).join("local"), temp_local_db).unwrap();
    }
    {
        let mut cmd = Command::new("fakeroot");
        cmd.args(["--", "pacman", "-Sy", "--dbpath", temp_db_dir]);
        execute(&mut cmd);
    }
    {
        let mut cmd = Command::new("pacman");
        cmd.args(["-Qu", "--dbpath", temp_db_dir]);
        execute(&mut cmd);
    }
}

fn paru() -> Command {
    Command::new("paru")
}

pub fn execute(cmd: &mut Command) {
    let output = cmd.output().unwrap();
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}
