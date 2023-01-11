use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::consts::PIXEL_SIZE;


pub fn setup_sdl() -> (Box<WindowCanvas>, Box<EventPump>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
        "rust-sdl2 demo",
        64 * PIXEL_SIZE,
        32 * PIXEL_SIZE
    )
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();

    (Box::new(canvas), Box::new(event_pump))
}