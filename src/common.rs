use core::arch::asm;
use core::fmt::Write;

pub type PAddrT = usize;
pub type VAddrT = usize;

// __builtin_align_up equivalent
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
// __builtin_is_aligned equivalent
pub fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}
// __builtin_offsetof equivalent
pub fn offset_of<T>(field: usize) -> usize {
    todo!();
}

pub fn memset(buf: *mut u8, c: u8, n: usize) -> *mut u8 {
    let mut p = buf;
    for _ in 0..n {
        unsafe {
            *p = c;
            p = p.add(1);
        }
    }
    buf
}

pub fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut d = dst;
    let mut s = src;
    for _ in 0..n {
        unsafe {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
        }
    }
    dst
}

pub fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut d = dst;
    let mut s = src;
    loop {
        unsafe {
            *d = *s;
            if *s == 0 {
                *d = b'\0';
                break;
            }
            d = d.add(1);
            s = s.add(1);
        }
    }
    dst
}

pub fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut p1 = s1;
    let mut p2 = s2;
    loop {
        unsafe {
            if *p1 == 0 && *p2 == 0 {
                return 0;
            }
            if *p1 < *p2 {
                return -1;
            }
            if *p1 > *p2 {
                return 1;
            }
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
    }
}

pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Self {}
    }
    pub fn putchar(&self, c: u8) {
        sbi_call(c as isize, 0, 0, 0, 0, 0, 0, 1).unwrap();
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.putchar(c);
        }
        Ok(())
    }
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

