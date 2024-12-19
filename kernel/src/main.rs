#![no_std]
#![no_main]

mod vga_buffer;


use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic_info:&PanicInfo)->!{
    loop {
        
    }
}

#[no_mangle]
pub extern "C" fn _start()->!{

      vga_buffer::print_something();
    // static HELLO:&[u8] = b"Hello World";
    // for (i,&byte) in HELLO.iter().enumerate(){
    //    let vga_buffer = 0xb8000 as *mut u8;
    //    unsafe{
    //    *vga_buffer.offset((i*2).try_into().unwrap()) = byte;
    //    *vga_buffer.offset((i*2+1).try_into().unwrap()) = code;
    //    }
    // }
    loop{

    }
}