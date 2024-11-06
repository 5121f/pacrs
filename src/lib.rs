mod pacman;

use pacman::Pacman;

pub fn list() {
    Pacman::new().list().run();
}
