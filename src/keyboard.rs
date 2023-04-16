use sdl2::rect::Point;
use specs::prelude::*;

use crate::components::*;
use super::MovementCommand;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let Some(movement_command) = &*data.0 else { return };

        (&data.1, &mut data.2).par_join()
        .for_each(|(_, vel)| {
            if let &MovementCommand::Move(direction, input) = movement_command {
                vel.direction = direction;
    
                if direction == Direction::Left || direction == Direction::Right {
                    vel.input.x = input;
                } else if direction == Direction::Up || direction == Direction::Down {
                    vel.input.y = input;
                }
            } else {
                vel.input = Point::new(0, 0)
            }
        });
    }
}
