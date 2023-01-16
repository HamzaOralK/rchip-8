use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;
use crate::consts::PIXEL_SIZE;

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn setup_sdl() -> (Box<WindowCanvas>, Box<AudioDevice<SquareWave>>, Box<EventPump>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap();

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

    (Box::new(canvas), Box::new(device), Box::new(event_pump))
}

pub fn draw(canvas: &mut WindowCanvas, gfx: [u8; 0x800],) {
    canvas.clear();

    let mut row = 0;
    for (index, element) in gfx.iter().enumerate() {
        if *element == 1 {
            canvas.set_draw_color(Color::WHITE);
        } else {
            canvas.set_draw_color(Color::BLACK);
        }
        canvas.fill_rect(Rect::new(((index as u32 % 64) * PIXEL_SIZE) as i32, (row * PIXEL_SIZE) as i32, PIXEL_SIZE, PIXEL_SIZE)).expect("Draw error");
        if (index + 1) % 64 == 0 {
            row += 1;
        }
    }

    canvas.present();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

pub fn get_chip8_keyboard_input(input: Keycode) -> Option<u8> {
    match input {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::Y => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::Num4 => Some(0xC),
        Keycode::R => Some(0xD),
        Keycode::F => Some(0xE),
        Keycode::V => Some(0xF),
        _ => None
    }
}