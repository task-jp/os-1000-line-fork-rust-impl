#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

fn sbi_call(
    arg0: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    fid: isize,
    eid: isize,
) -> Result<isize, isize> {
    let error: isize;
    let value: isize;
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") error,
            lateout("a1") value,
        );
    }
    if error < 0 {
        return Err(error);
    }
    Ok(value)
}

pub fn putchar(c: u8) {
    sbi_call(c as isize, 0, 0, 0, 0, 0, 0, 1).unwrap();
}

fn memset(buf: *mut u8, c: u8, n: usize) {
    let mut p = buf;
    for _ in 0..n {
        unsafe {
            *p = c;
            p = p.add(1);
        }
    }
}

#[allow(dead_code)]
fn kernel_main() {
    let bss_size = unsafe { &__bss_end as *const _ as usize - &__bss as *const _ as usize };
    unsafe { memset(&__bss as *const _ as *mut _, 0, bss_size) };

    for c in b"Hello, world!\n" {
        putchar(*c);
    }

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
