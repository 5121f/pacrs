mod pacman;

use pacman::Paru;

pub fn list() {
    Paru::new().list().run();
}

pub fn install(packages: Vec<String>) {
    Paru::new().install(packages).run()
}
