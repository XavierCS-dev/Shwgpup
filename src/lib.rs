mod run;
mod state;
mod texture;
mod vertex;
mod process_window_event;
mod sprite;
mod entity;

use crate::run::run;

struct Shmup {}

impl Shmup {
    fn update() {}
}

pub fn main_loop() {
    pollster::block_on(run());
}
