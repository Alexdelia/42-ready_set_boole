use crate::ex00::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut x = 0;
    for _ in 0..b {
        x = adder(x, a);
    }
    x
}
