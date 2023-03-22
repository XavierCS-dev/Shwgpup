mod entity;
mod process_window_event;
mod render_init;
mod run;
mod sprite;
mod state;
mod texture;
mod transformation;
mod vertex;
mod entity_group;

use crate::run::run;

struct Shmup {}

impl Shmup {
    fn update() {}
}

pub fn main_loop() {
    pollster::block_on(run());
}
