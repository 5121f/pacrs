use nix::unistd::getuid;
use which::which;

pub fn program_is_present(program: &str) -> bool {
    which(program).is_ok()
}

pub fn is_root() -> bool {
    getuid().is_root()
}
