use sdl2::rect::{Point, Rect};

use specs::*;
use specs_derive::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub input: Point,
    pub speed: i32,
    pub direction: Direction
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub down_frame: Vec<Sprite>,
    pub left_frame: Vec<Sprite>,
    pub right_frame: Vec<Sprite>,
    pub up_frame: Vec<Sprite>
}
