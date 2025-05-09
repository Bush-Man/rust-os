
use volatile::Volatile;
use core::fmt::*;
use lazy_static::*;
use spin::Mutex;
use core::fmt::Arguments;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color{
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
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode{
    pub fn new(background_color:Color,foreground_color:Color)->ColorCode{
       ColorCode((background_color as u8) << 4 | (foreground_color as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
   
    /**
     * The VGA buffer requires the data to be formatted as alternating bytes of ASCII character and color information,
     * Because we are writting directly to the VGA Buffer, we must follow its alignment
     */
    pub character : u8,
    pub color_code: ColorCode
}
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT:usize = 25;

#[repr(transparent)]
struct Buffer{
    chars: [[Volatile<ScreenChar>;BUFFER_WIDTH];BUFFER_HEIGHT]
}

pub struct Writer{
    column_position:usize,
    color_code:ColorCode,
    buffer: &'static mut Buffer
}
impl Writer{
    pub fn write_byte(&mut self,byte:u8){
        match byte{
            b'\n' => self.new_line(),
            byte =>{
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line()
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                self.buffer.chars[row][col].write( ScreenChar{
                    character:byte,
                    color_code:self.color_code
                });
                self.column_position += 1;
            }
        }
    }

  
    fn new_line(&mut self){
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let byte = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(byte);
                
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position=0;

    }
    fn clear_row(&mut self,row:usize){
        let blank = ScreenChar{
            character : b' ',
            color_code: self.color_code
        };
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }
    pub fn write_string(&mut self,s:&str){
        for byte in s.bytes(){
            match byte{
               0x20..=0x7e |  b'\n'=> self.write_byte(byte),
                _  => self.write_byte(0xfe)
            }
        }
    }
}
impl Write for Writer{
   fn write_str(&mut self,s:&str)->Result{
    self.write_string(s);
    Ok(())
   }
}

lazy_static!{
    pub static ref WRITER: Mutex<Writer> =Mutex::new(Writer{
        color_code: ColorCode::new(Color::Black,Color::Yellow),
        column_position:0,
        buffer: unsafe{&mut *(0xb8000 as *mut Buffer)}
    });
}


#[macro_export]
macro_rules! print{
    ($($arg:tt)*)=>($crate::vga_buffer::_print(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
    ()=>($crate::print!("\n"));

    ($($arg:tt)*)=>($crate::print!("{}\n",format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args:Arguments){
    WRITER.lock().write_fmt(args).unwrap();
}


// pub fn print_something(){
//     let mut writer = Writer{
//         color_code: ColorCode::new(Color::Black,Color::Yellow),
//        column_position:0,
//        buffer: unsafe{ &mut *(0xb8000 as *mut Buffer)}
//     };
//     writer.write_byte(b'H');
//     writer.write_string("ello ");
//     writer.write_string("Wörld!");
//     write!(writer,"The numbers are {} and {}",42,1.0/3.0).unwrap();

// }

