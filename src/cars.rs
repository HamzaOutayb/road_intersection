use std::borrow;
use std::cell::RefCell;

use sdl2::{render::Canvas, video::Window};

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(PartialEq, Debug)]
pub enum TrafficLight {
    UpperRight,
    UpperLeft,
    LowerRight,
    LowerLeft,
}

impl TrafficLight {
    pub fn change_traffic_light(self) -> Self {
        match self {
            TrafficLight::UpperRight => TrafficLight::UpperLeft,
            TrafficLight::UpperLeft => TrafficLight::LowerRight,
            TrafficLight::LowerRight => TrafficLight::LowerLeft,
            TrafficLight::LowerLeft => TrafficLight::UpperRight,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vehicles {
    pub vehicles: RefCell<Vec<RefCell<Vehicle>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub route: Route,
    pub direction: Direction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Vehicles {
    pub fn new() -> Self {
        Vehicles {
            vehicles: RefCell::new(Vec::new()),
        }
    }

    pub fn add_car(&mut self, direction: Direction) {
        let route = random_route();

        let new_vehicle = match direction {
            Direction::South => RefCell::new(Vehicle {
                route,
                direction,
                x: 350,
                y: 0,
            }),
            Direction::North => RefCell::new(Vehicle {
                route,
                direction,
                x: 400,
                y: 650,
            }),
            Direction::West => RefCell::new(Vehicle {
                route,
                direction,
                x: 800,
                y: 250,
            }),
            Direction::East => RefCell::new(Vehicle {
                route,
                direction,
                x: 0,
                y: 300,
            }),
        };

        if self
            .vehicles
            .borrow()
            .iter()
            .any(|v| intersects(&new_vehicle.borrow(), &v.borrow()))
            // || self.vehicles.borrow().len() >= 8
        {
            return;
        };

        self.vehicles.borrow_mut().push(new_vehicle)
    }

    pub fn draw_cars(&mut self, canvas: &mut Canvas<Window>, current_light: &TrafficLight) {
        if self.vehicles.borrow().is_empty() {
            return;
        }

        for (index, vehicle) in &mut self.vehicles.borrow().iter().enumerate() {
            if vehicle.borrow().route == Route::Left {
                canvas.set_draw_color(Color::YELLOW);
            } else if vehicle.borrow().route == Route::Right {
                canvas.set_draw_color(Color::MAGENTA);
            } else {
                canvas.set_draw_color(Color::BLUE);
            }

            canvas
                .fill_rect(Rect::new(vehicle.borrow().x, vehicle.borrow().y, 50, 50))
                .unwrap();

            // stop sign //
            if vehicle.borrow().direction == Direction::South
                && vehicle.borrow().y == 200
                && *current_light != TrafficLight::UpperLeft
            {
                continue;
            }

            if vehicle.borrow().direction == Direction::North
                && vehicle.borrow().y == 350
                && *current_light != TrafficLight::LowerRight
            {
                continue;
            }

            if vehicle.borrow().direction == Direction::West
                && vehicle.borrow().x == 450
                && *current_light != TrafficLight::UpperRight
            {
                continue;
            }

            if vehicle.borrow().direction == Direction::East
                && vehicle.borrow().x == 300
                && *current_light != TrafficLight::LowerLeft
            {
                continue;
            }

            if vehicle.borrow().x >= 350
                && vehicle.borrow().direction == Direction::East
                && vehicle.borrow().route == Route::Right
            {
                vehicle.borrow_mut().y += 2;
            } else if vehicle.borrow().x >= 400
                && vehicle.borrow().direction == Direction::East
                && vehicle.borrow().route == Route::Left
            {
                vehicle.borrow_mut().y -= 2
            } else if vehicle.borrow().x <= 400
                && vehicle.borrow().direction == Direction::West
                && vehicle.borrow().route == Route::Right
            {
                vehicle.borrow_mut().y -= 2
            } else if vehicle.borrow().x <= 350
                && vehicle.borrow().direction == Direction::West
                && vehicle.borrow().route == Route::Left
            {
                vehicle.borrow_mut().y += 2
            }
            //working
            else if vehicle.borrow().y <= 300
                && vehicle.borrow().direction == Direction::North
                && vehicle.borrow().route == Route::Right
            {
                vehicle.borrow_mut().x += 2;
            } else if vehicle.borrow().y <= 250
                && vehicle.borrow().direction == Direction::North
                && vehicle.borrow().route == Route::Left
            {
                vehicle.borrow_mut().x -= 2
            } else if vehicle.borrow().y >= 250
                && vehicle.borrow().direction == Direction::South
                && vehicle.borrow().route == Route::Right
            {
                vehicle.borrow_mut().x -= 2
            } else if vehicle.borrow().y >= 300
                && vehicle.borrow().direction == Direction::South
                && vehicle.borrow().route == Route::Left
            {
                vehicle.borrow_mut().x += 2
            } else if vehicle.borrow().direction == Direction::North {
                vehicle.borrow_mut().y -= 2;
                if self
                    .vehicles
                    .borrow()
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &index)
                    .any(|(_, v)| intersects(&vehicle.borrow(), &v.borrow()))
                {
                    vehicle.borrow_mut().y += 2;
                };
            } else if vehicle.borrow().direction == Direction::South {
                vehicle.borrow_mut().y += 2;
                if self
                    .vehicles
                    .borrow()
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &index)
                    .any(|(_, v)| intersects(&vehicle.borrow(), &v.borrow()))
                {
                    vehicle.borrow_mut().y -= 2;
                };
            } else if vehicle.borrow().direction == Direction::East {
                vehicle.borrow_mut().x += 2;
                if self
                    .vehicles
                    .borrow()
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &index)
                    .any(|(_, v)| intersects(&vehicle.borrow(), &v.borrow()))
                {
                    vehicle.borrow_mut().x -= 2;
                };
            } else if vehicle.borrow().direction == Direction::West {
                vehicle.borrow_mut().x -= 2;
                if self
                    .vehicles
                    .borrow()
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &index)
                    .any(|(_, v)| intersects(&vehicle.borrow(), &v.borrow()))
                {
                    vehicle.borrow_mut().x += 2;
                };
            }
            // let (width, height) = canvas.window().size();
            // let mut borrowed = self.vehicles.borrow_mut();
            // match vehicle.borrow().direction {
            //     Direction::North => {
            //         if vehicle.borrow().y < 0 {
            //             borrowed.remove(index);
            //         }
            //     }
            //     Direction::South => {
            //         if vehicle.borrow().y > height as i32 {
            //             borrowed.remove(index);
            //         }
            //     }
            //     Direction::East => {
            //         if vehicle.borrow().x > width as i32 {
            //             borrowed.remove(index);
            //         }
            //     }
            //     Direction::West => {
            //         if vehicle.borrow().x < 0 {
            //             borrowed.remove(index);
            //         }
            //     }
            // }
        }
    }

    pub fn add_random_car(&mut self) {
        let direction = match get_random_num(3) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        };

        self.add_car(direction);
    }
}

fn random_route() -> Route {
    match get_random_num(2) {
        0 => Route::Left,
        1 => Route::Right,
        2 => Route::Straight,
        _ => unreachable!(),
    }
}

pub fn get_random_num(max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(0..=max)
}

fn intersects(a: &Vehicle, b: &Vehicle) -> bool {
    let size = 70;

    let a_left = a.x;
    let a_right = a.x + size;
    let a_top = a.y;
    let a_bottom = a.y + size;

    let b_left = b.x;
    let b_right = b.x + size;
    let b_top = b.y;
    let b_bottom = b.y + size;

    !(a_left >= b_right
        || a_right <= b_left
        || a_top >= b_bottom
        || a_bottom <= b_top
        || a.direction != b.direction
    )
}
