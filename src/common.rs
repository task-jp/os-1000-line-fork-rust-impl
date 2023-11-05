use core::arch::asm;
use core::fmt::Write;

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

