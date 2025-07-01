use crate::models::prelude::*;

pub fn draw_intersection(canvas: &mut Canvas<Window>, width: u32, height: u32) {
    let road_width = 120;
    let center_x = (width / 2) as i32;
    let center_y = (height / 2) as i32;

    canvas.set_draw_color(Color::RGB(50, 50, 50));
    let _ = canvas.fill_rect(Rect::new(
        0,
        center_y - road_width / 2,
        width,
        road_width as u32,
    ));

    let _ = canvas.fill_rect(Rect::new(
        center_x - road_width / 2,
        0,
        road_width as u32,
        height,
    ));
    canvas.set_draw_color(WHITE);
    let _ = canvas.draw_line(
        Point::new((width / 2) as i32, 0),
        Point::new((width / 2) as i32, height as i32),
    );
    let _ = canvas.draw_line(
        Point::new(0, (height / 2) as i32),
        Point::new(width as i32, (height / 2) as i32),
    );

    let light1 = Light::new(Point::new(470, 370));
    let light2 = Light::new(Point::new(470, 230));
    let light3 = Light::new(Point::new(330, 370));
    let light4 = Light::new(Point::new(330, 230));

    light1.draw(canvas);
    light2.draw(canvas);
    light3.draw(canvas);
    light4.draw(canvas);
}
