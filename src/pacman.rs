use core::str;
use std::{fs, path::Path, process::Command};

use alpm::Alpm;
use alpm_utils::DbListExt;

const TEMP_DB_PATH: &str = "/tmp/pacrs/db";

pub fn list() {
    let mut cmd = paru();
    cmd.arg("-Qq");
    execute(&mut cmd)
}

fn alpm_with_db_path(db_path: &str) -> Alpm {
    let conf = pacmanconf::Config::new().unwrap();
    let mut alpm = Alpm::new(&*conf.root_dir, db_path).unwrap();
    alpm_utils::configure_alpm(&mut alpm, &conf).unwrap();
    alpm
}

fn alpm() -> Alpm {
    let conf = pacmanconf::Config::new().unwrap();
    alpm_utils::alpm_with_conf(&conf).unwrap()
}

fn package_was_updated_in_db(pkg: &str) -> bool {
    let alpm = alpm();
    let dbs = alpm.syncdbs();
    let alpm_temp = alpm_with_db_path(TEMP_DB_PATH);
    let dbs_temp = alpm_temp.syncdbs();
    let pkg_tmp = dbs_temp.pkg(pkg).unwrap();
    let pkg = dbs.pkg(pkg).unwrap();
    if pkg.version() < pkg_tmp.version() {
        return true;
    }
    false
}

pub fn install(packages: Vec<String>) {
    update_temp_db();
    for pkg in &packages {
        if package_was_updated_in_db(pkg) {
            println!("One or more package you will want to install was updated in the repo. Upgrade your system befor install it.");
            return;
        }
    }
    let mut cmd = paru();
    cmd.arg("-S").args(packages);
    execute(&mut cmd)
}

pub fn check_for_updates() {
    update_temp_db();
    let mut cmd = Command::new("pacman");
    cmd.args(["-Qu", "--dbpath", TEMP_DB_PATH]);
    execute(&mut cmd);
}

fn update_temp_db() {
    fs::create_dir_all(TEMP_DB_PATH).unwrap();
    let conf = pacmanconf::Config::new().unwrap();
    let temp_local_db = Path::new(TEMP_DB_PATH).join("local");
    if !temp_local_db.exists() {
        std::os::unix::fs::symlink(Path::new(&conf.db_path).join("local"), temp_local_db).unwrap();
    }

    let mut cmd = Command::new("fakeroot");
    cmd.args(["--", "pacman", "-Sy", "--dbpath", TEMP_DB_PATH]);
    execute(&mut cmd);
}

fn paru() -> Command {
    Command::new("paru")
}

pub fn execute(cmd: &mut Command) {
    cmd.spawn().unwrap().wait().unwrap();
}
