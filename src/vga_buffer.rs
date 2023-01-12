use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]

#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color)-> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }   
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
    character_value: u8,
    color_code: ColorCode
}


#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {


    pub fn write_byte_with_color(&mut self, byte: u8, color: ColorCode) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[BUFFER_HEIGHT - row + 1][col].write(ScreenChar {
                    character_value: byte,
                    color_code:color
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.write_byte_with_color(byte, self.color_code);
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) { 
        let blank = ScreenChar {
            character_value: b' ',
            color_code: self.color_code
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // Value out of range of supported chatracters 
            }
        }
    }

    fn write_string_with_color(&mut self, s: &str, color: ColorCode) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte_with_color(byte, color),
                _ => self.write_byte_with_color(0xfe, color), // Value out of range of supported chatracters 
            }
        }
    }

}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}



// Redefining the print / println macros since we have ommited the STD library.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Extra macros for printing errors
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::vga_buffer::_error(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error_nl {
    () => ($crate::error!("\n"));
    ($($arg:tt)*) => ($crate::error!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _error(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_string_with_color("Error! ", ColorCode::new(Color::Red, Color::Black));
    WRITER.lock().write_fmt(args).unwrap();
}