use crate::cmds::PARU_BIN;

use nix::unistd::getuid;
use which::which;

pub fn program_is_present() -> bool {
    which(PARU_BIN).is_ok()
}

pub fn is_root() -> bool {
    getuid().is_root()
}
