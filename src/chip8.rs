use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::consts::PIXEL_SIZE;

const CHIP8_FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    memory: [u8; 0x1000],
    cpu_register: [u8; 0x10],
    i_register: u16,
    program_counter: u16,
    gfx: [u8; 0x800],
    pub(crate) is_draw: bool
}

impl Chip8 {
    pub fn initialize() -> Self {
        let mut memory = [0; 0x1000];

        for (index, element) in CHIP8_FONT_SET.iter().enumerate() {
            memory[index] = *element
        };

        Self {
            memory,
            cpu_register: [0; 0x10],
            i_register: 0x0,
            program_counter: 0x200,
            gfx: [0; 0x800],
            is_draw: false
        }
    }

    fn clear_screen(&mut self) {
        self.gfx = [0; 0x800];
    }

    pub fn load_rom(&mut self, buffer:  [u8; 0xE00]){
        for (index, element) in buffer.iter().enumerate() {
            self.memory[512 + index] = *element;
        }
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    pub fn cycle(&mut self) {
        let op_code = u16::from(self.memory[self.program_counter as usize]) << 8 | u16::from(self.memory[self.program_counter as usize + 1]);

        match op_code & 0xF000 {
            0x0000 => match op_code & 0x00FF {
                0x00E0 => {
                    self.clear_screen();
                    self.increment_program_counter();
                },
                0x00EE => {

                }
                _ => {}
            },
            0x1000 => {
                self.program_counter = op_code & 0x0FFF;
            },
            0x6000 => {
                let index = ((op_code & 0x0F00) >> 8) as usize;
                self.cpu_register[index] = (op_code & 0x00FF) as u8;
                self.increment_program_counter();
            },
            0x7000 => {
                let index = ((op_code & 0x0F00) >> 8) as usize;
                let addition = (op_code & 0x00FF) as u8;
                self.cpu_register[index] += addition;
                self.increment_program_counter();
            },
            0xA000 => {
                self.i_register = op_code & 0x0FFF;
                self.increment_program_counter();
            },
            0xB000 => {
                self.program_counter = u16::from(self.cpu_register[0]) + (op_code * 0x0FFF);
            },
            0xC000 => {
                let random_number: u8 = rand::thread_rng().gen();
                let index = ((op_code & 0x0F00) >> 8) as usize;
                self.cpu_register[index] = (u16::from(random_number) & (op_code & 0x00FF)) as u8;
                self.increment_program_counter();
            },
            0xD000 => {
                let x = self.cpu_register[((op_code & 0x0F00) >> 8) as usize] as u16;
                let y = self.cpu_register[((op_code & 0x00F0) >> 4) as usize] as u16;
                let height = op_code & 0x000F;
                let mut pixel: u16;

                self.cpu_register[0xF] = 0;
                for y_line in 0..height {
                    pixel = self.memory[(self.i_register + y_line) as usize] as u16;
                    for x_line in 0..8 {
                        if (pixel & (0x80 >> x_line)) != 0 {
                            if self.gfx[(x + x_line + ((y + y_line) * 64)) as usize] == 1 {
                                self.cpu_register[0xF] = 1
                            }
                            self.gfx[(x + x_line + ((y + y_line) * 64)) as usize] ^= 1;
                        }
                    }
                }
                self.is_draw = true;
                self.increment_program_counter();
            }
            _ => println!("Unknown code")
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.clear();

        let mut row = 0;
        for (index, element) in self.gfx.iter().enumerate() {
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
}