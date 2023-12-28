use std::{time::{Duration, Instant}, sync::RwLock};

pub static TIME: RwLock<Option<Instant>> = RwLock::new(None);

pub static COLORS: [u32; 4] = [
    0xFFFFFFFF,
    0xFFAAAAAA,
    0xFF555555,
    0xFF000000
];

pub fn bit(a: u8, n: u8) -> bool {
    (a & (1 << n)) != 0
}

pub fn bit_set(a: &mut u8, n: u8, on: bool) {
    // Flip bit on
    if on {
        *a |= 1 << n;
        return;
    }

    // Flip bit off
    *a &= !(1 << n);
}

pub fn between(a: u16, b: u16, c: u16) -> bool {
    b <= a && a <= c
}

pub fn delay(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
