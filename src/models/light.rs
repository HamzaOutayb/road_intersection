use crate::models::prelude::*;

#[derive(Clone)]
pub struct Light {
    pub position: Point,
    radius: u32,
    color: Color,
    pub is_on: bool,
    pub timer: u32,
}

impl Light {
    pub fn new(position: Point) -> Self {
        Self {
            position,
            radius: 10,
            color: Color::RGB(255, 0, 0),
            is_on: false,
            timer: 0,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        self.color = Color::RGB(0, 255, 0);
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.color = Color::RGB(255, 0, 0);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        for angle in 0..360 {
            let radian = (angle as f64).to_radians();
            let x = self.position.x + (radian.cos() * self.radius as f64) as i32;
            let y = self.position.y + (radian.sin() * self.radius as f64) as i32;
            canvas.draw_point(Point::new(x, y)).unwrap();
        }
    }
}
