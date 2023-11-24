use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::{prelude::*, gamepad::GamepadEvent};
use std::time::Duration;
use rand::Rng;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread};
use console::{Term, Key};

mod gemini;

use gemini::{GeminiPlugin, Blob};
use gemini_engine::elements::{Vec2D, view::ColChar};

const FPS: f64 = 10.0;

const CANVAS_W: usize = 20;
const CANVAS_H: usize = 8;

fn spawn_stdin_channel() -> Receiver<Key> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let term = Term::stdout();
        loop {
            if let Ok(key) = term.read_key() {
                tx.send(key).unwrap();
            }
        }
    });
    rx
}

fn insert_console(world: &mut World) {
    let rx = spawn_stdin_channel();
    world.insert_non_send_resource(rx);
}

fn console_input(
    mut query: Query<&mut Blob, With<Player>>,
    rx_channel: NonSend<Receiver<Key>>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        if let Ok(key) = rx_channel.try_recv() {
            match key {
                Key::Char(c) => {
                    match c {
                        'a' => player.pixel.pos.x -= 1,
                        'd' => player.pixel.pos.x += 1,
                        'w' => player.pixel.pos.y -= 1,
                        's' => player.pixel.pos.y += 1,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}



#[derive(Component)]
struct Player;

// Gamepad stuff (need to actually connect a gamepad for this to work)
fn gamepad_connections(
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for event in gamepad_evr.read() {
        match event {
            GamepadEvent::Connection(info) => {
                println!("{:?}", info);
            }
            _ => {}
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Blob, With<Player>>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::A) {
            player.pixel.pos.x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            player.pixel.pos.x += 1;
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            player.pixel.pos.y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            player.pixel.pos.y += 1;
        }
    }
}


fn main() {
    // This app runs once
    App::new()
        // Basically MinimalPlugins
        .add_plugins(bevy_core::TaskPoolPlugin::default())
        .add_plugins(bevy_core::TypeRegistrationPlugin)
        .add_plugins(bevy_core::FrameCountPlugin)
        .add_plugins(bevy_time::TimePlugin)
        .add_plugins(bevy_input::InputPlugin)
        // Use a fixed framerate
        .add_plugins(bevy_app::ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / FPS),
        ))
        // Console
        .add_systems(Startup, insert_console)
        .add_systems(Update, console_input)
        // User systems and plugins
        .add_plugins(GeminiPlugin {
            canvas_w: CANVAS_W,
            canvas_h: CANVAS_H,
        })
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, gamepad_connections)
        .add_systems(Startup, init_pixels)
        // Run app
        .run();
}

fn init_pixels(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Blob::new(Vec2D::new(3, 2), ColChar::SOLID.with_rgb(30, 200, 30)),
    ));

    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let x = rng.gen_range(0..CANVAS_W-1);
        let y = rng.gen_range(0..CANVAS_H-1);
        commands.spawn(
            Blob::new(Vec2D::new(x as isize, y as isize), ColChar::SOLID.with_rgb(200, 30, 30)),
        );
    }
}


