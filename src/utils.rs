use crate::cmds::PARU_BIN;

use which::which;

pub fn program_is_present() -> bool {
    which(PARU_BIN).is_ok()
}
