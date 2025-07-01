use crate::models::prelude::*;

pub fn create_window_and_canvas(
    sdl_context: &Sdl,
    title: &str,
    width: u32,
    height: u32,
) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();

    window.into_canvas().build().unwrap()
}
