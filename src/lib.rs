mod pacman;

use pacman::Pacman;

pub fn list() {
    Pacman::new().list().run();
}

pub fn install(packages: Vec<String>) {
    Pacman::new().install(packages).run()
}
