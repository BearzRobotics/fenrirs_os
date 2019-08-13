#![no_std] // disable standard lib
#![no_main] // disable fn main and c runtime. Must supply our own _start


use core::panic::PanicInfo;

mod vga_buffer;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello");
    panic!("Some panic message");
    loop {}
}
