mod constants;
mod models;
mod util;

use crate::models::prelude::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = create_window_and_canvas(&sdl_context, "Road Intersection", 800, 600);

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let (width, height) = canvas.window().size();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => {
                    let car = Car::new(Spawn::South);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let car = Car::new(Spawn::North);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let car = Car::new(Spawn::West);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let car = Car::new(Spawn::East);
                }
                _ => {}
            }
        }

        draw_intersection(&mut canvas, width, height);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
