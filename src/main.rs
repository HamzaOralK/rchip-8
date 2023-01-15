mod chip8;
mod sdl2_utility;
mod consts;

use std::fs::File;
use std::io::{Read};
use std::thread::sleep;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::sdl2_utility::{get_chip8_keyboard_input, setup_sdl};
use crate::chip8::Chip8;

fn main() {
    let mut f = File::open("./roms/Keypad Test.ch8").expect("File not found");
    // let mut f = File::open("./roms/IBM Logo.ch8").expect("File not found");
    let mut buffer = [0; 0xE00];

    let _ = f.read(&mut buffer).expect("Read error");

    let mut chip = Chip8::initialize();
    chip.load_rom(buffer);

    let (mut canvas, mut event_pump) = setup_sdl();

    'running: loop {
        chip.cycle();

        if chip.is_draw {
            sdl2_utility::draw(&mut canvas, chip.gfx);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(code), .. } => {
                    chip.set_keyboard_input(get_chip8_keyboard_input(code));
                },
                Event::KeyUp{..} => {
                    chip.set_keyboard_input(None);
                }
                _ => {}
            }
        }

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
