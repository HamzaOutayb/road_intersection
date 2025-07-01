use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::{render::Canvas, video::Window};
use sdl2::rect::Point;

mod cars;
use cars::*;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("road_intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut all_vehicles = Vehicles::new();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                        match key {
                            Keycode::Up => {
                                all_vehicles.add_car(Direction::North);
                            }
                            Keycode::Down => {
                                all_vehicles.add_car(Direction::South);
                            }
                            Keycode::Left => {
                                all_vehicles.add_car(Direction::East);
                            }
                            Keycode::Right => {
                                all_vehicles.add_car(Direction::West);
                            }
                            Keycode::R => {
                                all_vehicles.add_random_car();
                            }
                            _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        draw_roads(&mut canvas);
        all_vehicles.draw_cars(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
}

pub fn draw_roads(canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.output_size().unwrap();
        let height=  height as i32;
        let width=  width as i32;

        let center_x = width / 2;
        let center_y = height / 2;

        canvas.set_draw_color(Color::WHITE);

        canvas.draw_line(Point::new(center_x, 0), Point::new(center_x, height)).unwrap();
        canvas.draw_line(Point::new(center_x + 50, 0), Point::new(center_x + 50, height)).unwrap();
        canvas.draw_line(Point::new(center_x - 50, 0), Point::new(center_x - 50, height)).unwrap();

        canvas.draw_line(Point::new(0, center_y), Point::new(width, center_y)).unwrap();
        canvas.draw_line(Point::new(0, center_y + 50), Point::new(width, center_y + 50)).unwrap();
        canvas.draw_line(Point::new(0, center_y - 50), Point::new(width, center_y - 50)).unwrap();

}
