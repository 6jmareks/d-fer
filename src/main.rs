#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        //gee whiz i sure do hope it doesnt break

    }

}
static message1: &[u8] = b"Welcome to ";
static message2: &[u8] = b"DFER!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //kernel entry point
    
    let mut glob_pointer_position: usize = 0;
    print(message1, 0x0f, &mut glob_pointer_position);
    print(message2, 0x0a, &mut glob_pointer_position);


    loop {
        
        

    }
}


fn print(msg: &[u8], colour: u8, current_pointer_position: &mut usize) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in msg.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(*current_pointer_position as isize) = byte;
                *vga_buffer.offset(*current_pointer_position as isize + 1) = colour;
            }
            *current_pointer_position += 2;
        
        }
    
}