use rand::Rng;

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
    pub(crate) gfx: [u8; 0x800],
    stack: [u16; 0xC],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    pub is_sound: bool,
    pub is_draw: bool,
    pub keyboard_input: Option<u8>
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
            is_draw: false,
            is_sound: false,
            stack: [0; 0xC],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keyboard_input: None
        }
    }

    pub fn set_keyboard_input(&mut self, input: Option<u8>) {
        self.keyboard_input = input;
    }

    fn clear_screen(&mut self) {
        self.gfx = [0; 0x800];
    }

    pub fn load_rom(&mut self, buffer:  &[u8]){
        for (index, element) in buffer.iter().enumerate() {
            self.memory[512 + index] = *element;
        }
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    fn skip_next_instruction(&mut self) {
        self.program_counter += 4;
    }

    pub fn cycle(&mut self) {
        let op_code = u16::from(self.memory[self.program_counter as usize]) << 8 | u16::from(self.memory[self.program_counter as usize + 1]);
        let x_index = ((op_code & 0x0F00) >> 8) as usize;
        let y_index = ((op_code & 0x00F0) >> 4) as usize;

        match op_code & 0xF000 {
            0x0000 => match op_code & 0x00FF {
                0x00E0 => {
                    self.clear_screen();
                    self.increment_program_counter();
                },
                0x00EE => {
                    self.program_counter = self.stack[self.stack_pointer];
                    self.stack_pointer -= 1;
                }
                _ => { println!("Unknown code: {}", op_code) }
            },
            0x1000 => {
                self.program_counter = op_code & 0x0FFF;
            },
            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.program_counter + 2;
                self.program_counter = op_code & 0x0FFF;
            },
            0x3000 => {
                if self.cpu_register[x_index] == ((op_code & 0x00FF) as u8) {
                    self.skip_next_instruction();
                } else {
                    self.increment_program_counter();
                }
            },
            0x4000 => {
                if self.cpu_register[x_index] != ((op_code & 0x00FF) as u8) {
                    self.skip_next_instruction();
                } else {
                    self.increment_program_counter();
                }
            },
            0x5000 => {
              if self.cpu_register[x_index] == self.cpu_register[y_index] {
                  self.skip_next_instruction();
              } else {
                  self.increment_program_counter();
              }
            },
            0x6000 => {
                self.cpu_register[x_index] = (op_code & 0x00FF) as u8;
                self.increment_program_counter();
            },
            0x7000 => {
                let addition = (op_code & 0x00FF) as u8;
                self.cpu_register[x_index] = self.cpu_register[x_index].wrapping_add(addition);
                self.increment_program_counter();
            },
            0x8000 => match op_code & 0x000F {
                0x0000 => {
                    self.cpu_register[x_index] = self.cpu_register[y_index];
                    self.increment_program_counter();
                },
                0x0001 => {
                    self.cpu_register[x_index] |= self.cpu_register[y_index];
                    self.increment_program_counter();
                },
                0x0002 => {
                    self.cpu_register[x_index] &= self.cpu_register[y_index];
                    self.increment_program_counter();
                },
                0x0003 => {
                    self.cpu_register[x_index] ^= self.cpu_register[y_index];
                    self.increment_program_counter();
                },
                0x0004 => {
                    if let Some(c) = self.cpu_register[x_index].checked_add(self.cpu_register[y_index]) {
                        self.cpu_register[x_index] = c;
                        self.cpu_register[0xF] = 0;
                    } else {
                        self.cpu_register[x_index] = self.cpu_register[x_index].wrapping_add(self.cpu_register[y_index]);
                        self.cpu_register[0xF] = 1;
                    }
                    self.increment_program_counter();
                },
                0x0005 => {
                    if let Some(c) = self.cpu_register[x_index].checked_sub(self.cpu_register[y_index]) {
                        self.cpu_register[x_index] = c;
                        self.cpu_register[0xF] = 1;
                    } else {
                        self.cpu_register[x_index] = self.cpu_register[x_index].wrapping_sub(self.cpu_register[y_index]);
                        self.cpu_register[0xF] = 0;
                    }
                    self.increment_program_counter();
                },
                0x0006 => {
                    self.cpu_register[0xF] = self.cpu_register[x_index] & 0x1;
                    self.cpu_register[x_index] >>= 1;
                    self.increment_program_counter();
                },
                0x0007 => {
                    if let Some(c) = self.cpu_register[y_index].checked_sub(self.cpu_register[x_index]) {
                        self.cpu_register[x_index] = c;
                        self.cpu_register[0xF] = 1;
                    } else {
                        self.cpu_register[x_index] = self.cpu_register[y_index].wrapping_sub(self.cpu_register[x_index]);
                        self.cpu_register[0xF] = 0;
                    }
                    self.increment_program_counter();
                },
                0x000E => {
                    self.cpu_register[0xF] = (self.cpu_register[x_index] >> 7) & 0x1;
                    self.cpu_register[x_index] <<= 1;
                    self.increment_program_counter();
                },
                _ => println!("{} with 8", op_code)
            },
            0x9000 => {
              if self.cpu_register[x_index] != self.cpu_register[y_index] {
                  self.skip_next_instruction();
              } else {
                  self.increment_program_counter();
              }
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
                self.cpu_register[x_index] = (u16::from(random_number) & (op_code & 0x00FF)) as u8;
                self.increment_program_counter();
            },
            0xD000 => {
                let x = self.cpu_register[x_index] as u16;
                let y = self.cpu_register[y_index] as u16;

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
            },
            0xE000 => match op_code & 0x00FF {
                0x009E => {
                    if self.keyboard_input == Some(self.cpu_register[x_index]) {
                        self.skip_next_instruction();
                    } else {
                        self.increment_program_counter();
                    }
                },
                0x00A1 => {
                    if self.keyboard_input != Some(self.cpu_register[x_index]) {
                        self.skip_next_instruction();
                    } else {
                        self.increment_program_counter();
                    }
                },
                _ => {}
            }
            0xF000 => {
                match op_code & 0x00FF {
                    0x0007 => {
                        self.cpu_register[x_index] = self.delay_timer;
                        self.increment_program_counter();
                    },
                    0x000A => {
                        if let Some(i) = self.keyboard_input {
                            self.cpu_register[x_index] = i;
                            self.increment_program_counter();
                        }
                    },
                    0x0015 => {
                        self.delay_timer = self.cpu_register[x_index];
                        self.increment_program_counter();
                    },
                    0x0018 => {
                        self.sound_timer = self.cpu_register[x_index];
                        self.increment_program_counter();
                    },
                    0x001E => {
                        self.i_register += u16::from(self.cpu_register[x_index]);
                        self.increment_program_counter();
                    },
                    0x0029 => {
                        self.i_register = (5 * self.cpu_register[x_index]) as u16;
                        self.increment_program_counter();
                    },
                    0x0033 => {
                        self.memory[self.i_register as usize] = (self.cpu_register[x_index] % 100) / 100;
                        self.memory[(self.i_register + 1) as usize] = (self.cpu_register[x_index] % 100) / 10;
                        self.memory[(self.i_register + 2) as usize] = self.cpu_register[x_index] % 10;
                        self.increment_program_counter();
                    },
                    0x0055 => {
                        for i in 0..=x_index {
                            self.memory[self.i_register as usize + i] = self.cpu_register[i];
                        }
                        self.increment_program_counter();
                    }
                    0x0065 => {
                        for i in 0..=x_index {
                            self.cpu_register[i] = self.memory[self.i_register as usize + i];
                        }
                        self.increment_program_counter();
                    },
                    _ => println!("Unknown code: {}", op_code)
                }
            }
            _ => println!("Unknown code: {}", op_code)
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                self.is_sound = true;
            }
            self.sound_timer -= 1;
        } else {
            self.is_sound = false;
        }
    }
}