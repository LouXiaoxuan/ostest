#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), "the paper should be accepted!").unwrap();
    println!("hello world{}","!");

    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();

    // #[cfg(test)]
    // test_main();

    println!("It did not crash!");
    loop{}
}
