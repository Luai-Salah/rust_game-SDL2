mod components;
mod movement;
mod animator;
mod keyboard;
mod renderer;

use std::time::Duration;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};

use specs::prelude::*;

use components::*;

pub enum MovementCommand {
    Stop,
    Move(Direction, i32),
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use Direction::*;

    match direction {
        Down => 0,
        Left => 1,
        Right => 2,
        Up => 3
    }
}

fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height
            )
        })
    }

    frames
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Game Tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let texture_creator = canvas.texture_creator();

    let textures = [
        texture_creator.load_texture("assets/bardo.png")?
    ];

    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation {
        current_frame: 0,
        down_frame: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frame: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frame: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
        up_frame: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
    };

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(movement::Movement, "Movement", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
    .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {speed: 5, direction: Direction::Down, input: Point::new(0, 0)})
        .with(player_animation.down_frame[0].clone())
        .with(player_animation)
    .build();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {

        let mut movement_command = None;
        // Handle Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left, -1));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right, 1));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up, -1));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down, 1));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                },
                
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                },
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch_par(&mut world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}