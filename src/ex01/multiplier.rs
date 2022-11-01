use crate::ex00::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut x = 0;
    for i in 0..32 {
        if (b & (1 << i)) != 0 {
            x = adder(x, a << i);
        }
    }
    x
}
