use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::{render::Canvas, video::Window};
use std::time::{Duration, Instant};

mod cars;
use crate::cars::{Direction, TrafficLight, Vehicles};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road_intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut all_vehicles = Vehicles::new();

    let mut traffic_light = TrafficLight::UpperRight;
    let mut last_change = Instant::now();
    let change_interval = Duration::from_secs(5);

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
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Up => all_vehicles.add_car(Direction::North),
                    Keycode::Down => all_vehicles.add_car(Direction::South),
                    Keycode::Left => all_vehicles.add_car(Direction::West),
                    Keycode::Right => all_vehicles.add_car(Direction::East),
                    Keycode::R => all_vehicles.add_random_car(),
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        draw_roads(&mut canvas, &traffic_light);
        all_vehicles.draw_cars(&mut canvas, &traffic_light);

        canvas.present();

        if last_change.elapsed() >= change_interval {
            traffic_light = traffic_light.change_traffic_light();
            last_change = Instant::now();
        }

        ::std::thread::sleep(Duration::from_micros(16_666));
    }
}

// Now draw_roads takes &TrafficLight instead of TrafficLight by value
fn draw_roads(canvas: &mut Canvas<Window>, current_light: &TrafficLight) {
    let (width, height) = canvas.output_size().unwrap();
    let width = width as i32;
    let height = height as i32;

    let center_x = width / 2;
    let center_y = height / 2;

    // Road lines
    canvas.set_draw_color(Color::WHITE);
    canvas
        .draw_line(Point::new(center_x, 0), Point::new(center_x, height))
        .unwrap();
    canvas
        .draw_line(
            Point::new(center_x + 50, 0),
            Point::new(center_x + 50, height),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(center_x - 50, 0),
            Point::new(center_x - 50, height),
        )
        .unwrap();

    canvas
        .draw_line(Point::new(0, center_y), Point::new(width, center_y))
        .unwrap();
    canvas
        .draw_line(
            Point::new(0, center_y + 50),
            Point::new(width, center_y + 50),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(0, center_y - 50),
            Point::new(width, center_y - 50),
        )
        .unwrap();

    // Traffic lights positions
    let positions = [
        (
            TrafficLight::UpperRight,
            Rect::new(center_x + 50, center_y - 100, 50, 50),
        ),
        (
            TrafficLight::UpperLeft,
            Rect::new(center_x - 100, center_y - 100, 50, 50),
        ),
        (
            TrafficLight::LowerRight,
            Rect::new(center_x + 50, center_y + 50, 50, 50),
        ),
        (
            TrafficLight::LowerLeft,
            Rect::new(center_x - 100, center_y + 50, 50, 50),
        ),
    ];

    // Draw traffic lights, green for active, red otherwise
    for (light, rect) in positions {
        if light == *current_light {
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }
        canvas.fill_rect(rect).unwrap();
    }
}
