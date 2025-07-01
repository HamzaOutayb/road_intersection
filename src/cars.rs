use sdl2::{render::Canvas, video::Window};

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;

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

#[derive(Debug)]
pub struct Vehicles {
    pub vehicles: Vec<Vehicle>
}

#[derive(Debug)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub route: Route,
    pub direction: Direction,
}

#[derive(Debug, PartialEq)]
pub enum Route {
Left,
Right,
Straight,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
North,
South,
East,
West,
}

impl Vehicles {
    pub fn new() -> Self {
        Vehicles {
            vehicles: Vec::new(),
        }
    }

    pub fn add_car(&mut self, direction: Direction) {
        let route = random_route();
        
        
       let new_vehicle = match direction {
        Direction::South => Vehicle {
            route,
            direction,
            x: 350,
            y: 0,
        },
        Direction::North => Vehicle {
            route,
            direction,
            x: 400,
            y: 650,
        },
        Direction::West => Vehicle {
            route,
            direction,
            x: 0,
            y: 300,
        },
        Direction::East => Vehicle {
            route,
            direction,
            x: 800,
            y: 250,
        },
    };

        if self.vehicles.iter().any(|v| intersects(&new_vehicle, v)) {
        return; 
        };

        self.vehicles.push(new_vehicle)
}

    pub fn draw_cars(&mut self, canvas: &mut Canvas<Window>) {
        if self.vehicles.len() == 0 { return }

        for vehicle in &mut self.vehicles {

        if vehicle.route == Route::Left {
            canvas.set_draw_color(Color::YELLOW);
        } else if vehicle.route == Route::Right {
            canvas.set_draw_color(Color::MAGENTA);
        } else {
            canvas.set_draw_color(Color::BLUE);
        }

        canvas.fill_rect(Rect::new(vehicle.x, vehicle.y,50, 50)).unwrap();

        if  vehicle.y <= 300 && vehicle.direction == Direction::North && vehicle.route == Route::Right {
            vehicle.x += 2
        } else if  vehicle.y <= 250 && vehicle.direction == Direction::North && vehicle.route == Route::Left {
            vehicle.x -= 2
        } else if  vehicle.y >= 250 && vehicle.direction == Direction::South && vehicle.route == Route::Right {
            vehicle.x -= 2
        } else if  vehicle.y >= 300 && vehicle.direction == Direction::South && vehicle.route == Route::Left {
            vehicle.x += 2
        } else if vehicle.direction == Direction::North {
            vehicle.y -= 2;
        } else if vehicle.direction == Direction::South {
            vehicle.y += 2;
        } else if vehicle.direction == Direction::East {
            vehicle.x -= 2;
        } else if vehicle.direction == Direction::West {
            vehicle.x += 2;
        }
        }

    }

    
        pub fn add_random_car(&mut self) {
        let mut rng = rand::thread_rng();

        let direction = match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        };

        self.add_car(direction);
    }
}

fn random_route() -> Route {
    let rng = rand::thread_rng().gen_range(0..3);
    match rng {
        0 => return Route::Left,
        1=> return Route::Right,
        2=> return Route::Straight,
        _=> unreachable!(),
    }
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

    !(a_left >= b_right ||
      a_right <= b_left ||
      a_top >= b_bottom ||
      a_bottom <= b_top)
}
