mod engine;
mod actors;
mod scenes;
use crate::engine::run::run;

struct Shmup {}

impl Shmup {
    fn update() {}
}

pub fn main_loop() {
    pollster::block_on(run());
}
