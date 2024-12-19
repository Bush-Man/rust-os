

fn main() {
    
#[derive(Debug)] 
#[repr(u8)]  
#[allow(dead_code)]
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
#[derive(Debug)] 
#[repr(transparent)]
struct ColorCode(u8);


impl ColorCode{
    pub fn new(background_color:Color,foreground_color:Color)->ColorCode{
        ColorCode((background_color as u8 )<< 4 | (foreground_color as u8))
    }
}
println!("{:?}", ColorCode::new(Color::Blue, Color::Green));


}
