mod chip8;
mod sdl2_setup;
mod consts;

use std::fs::File;
use std::io::{Read};
use std::thread::sleep;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::sdl2_setup::setup_sdl;
use crate::chip8::Chip8;

fn main() {
    let mut f = File::open("./roms/IBM Logo.ch8").expect("File not found");
    let mut buffer = [0; 0xE00];

    let _ = f.read(&mut buffer).expect("Read error");

    let mut chip = Chip8::initialize();
    chip.load_rom(buffer);

    let (mut canvas, mut event_pump) = setup_sdl();

    'running: loop {
        chip.cycle();

        if chip.is_draw {
            chip.draw(&mut canvas);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
