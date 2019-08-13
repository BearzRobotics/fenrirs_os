#![no_std] // disable standard lib
#![no_main] // disable fn main and c runtime. Must supply our own _start


use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello Wolrd!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello");

    loop {}
}
