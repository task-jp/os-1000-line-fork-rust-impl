#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt::Write;
mod common;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

#[allow(dead_code)]
fn kernel_main() {
    let bss_size = unsafe { &__bss_end as *const _ as usize - &__bss as *const _ as usize };
    unsafe { common::memset(&__bss as *const _ as *mut _, 0, bss_size) };

    let mut console = common::Console::new();
    writeln!(&mut console, "Hello, world!");
    writeln!(&mut console, "1 + 2 = {}", 1 + 2);

    loop {}
}


#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
    // スタックポインタを初期化
    asm!(
        "mv sp, {stack_top}\n
        j {kernel_main}\n",
        stack_top = in(reg) &__stack_top,
        kernel_main = sym kernel_main,
    );
    loop {}
}
