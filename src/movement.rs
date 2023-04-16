use specs::prelude::*;

use crate::components::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&mut data.0, &data.1).par_join().for_each(|(pos, vel)| {
            pos.0 += vel.input * vel.speed;
        });
    }
}
