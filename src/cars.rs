use sdl2::{render::Canvas, video::Window};

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;

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
        if direction == Direction::South {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 350,
                y: -50,
            });
        } else if  direction == Direction::North {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 400,
                y: 650,
            });
        } else if  direction == Direction::West {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 0,
                y: 300
            });
        } else if  direction == Direction::East {
            self.vehicles.push(Vehicle{
                route: route,
                direction: direction,
                x: 800,
                y: 250
            });
        }
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
    if rng == 0 {
        return Route::Left;
    } else if rng == 1 {
        return Route::Right;
    }
    Route::Straight
}
