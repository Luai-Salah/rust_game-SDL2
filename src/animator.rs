use sdl2::rect::Point;
use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        (&mut data.0, &mut data.1, &data.2).par_join()
        .for_each(|(anim, sprite, vel)| {
            if vel.input == Point::new(0, 0) {
                return;
            }

            let frames = match vel.direction {
                Direction::Down => &anim.down_frame,
                Direction::Left => &anim.left_frame,
                Direction::Right => &anim.right_frame,
                Direction::Up => &anim.up_frame,
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame].clone();
        });
    }
}
