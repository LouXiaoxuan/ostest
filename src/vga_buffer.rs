const BUFFER_HEIGHT:usize = 25;
const BUFFER_WIDTH:usize = 80;
use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color{
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode{
    fn new(frontcolor:Color, backcolor:Color) -> ColorCode{
        ColorCode((backcolor as u8) << 4 | (frontcolor as u8))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character:u8,
    color_code:ColorCode,
}

#[repr(transparent)]
struct Buffer{
    chars:[[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer{
    column_position:usize,
    color_code:ColorCode,
    buffer:&'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer{
    pub fn write_byte(&mut self, byte:u8){
        match byte{
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT-1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character:byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s:&str){
        for byte in s.bytes(){
            match byte{
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
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
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

lazy_static!{
    pub static ref WRITER:Mutex<Writer> = Mutex::new(Writer{
        column_position:0,
        color_code:ColorCode::new(Color::Yellow, Color::Black),
        buffer:unsafe{&mut *(0xb8000 as *mut Buffer)},
    });
}

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


