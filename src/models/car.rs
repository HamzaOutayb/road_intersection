use crate::models::prelude::*;

enum Color {
    Yellow,
    Magenta,
    Blue,
}

enum Direction {
    Left,
    Right,
    Straight,
}

pub enum Spawn {
    North,
    South,
    East,
    West,
}

pub struct Car {
    color: Color,
    direction: Direction,
    spawn: Spawn,
}

impl Car {
    pub fn new(spawn: Spawn) -> Self {
        match get_random_num(3) {
            0 => Self {
                color: Color::Yellow,
                direction: Direction::Left,
                spawn,
            },
            1 => Self {
                color: Color::Magenta,
                direction: Direction::Right,
                spawn,
            },
            _ => Self {
                color: Color::Blue,
                direction: Direction::Straight,
                spawn,
            },
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        todo!()
    }
}
