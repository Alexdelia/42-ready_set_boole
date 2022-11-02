use ready_set_boole::ex02::gray_code::gray_code;
use std::env::args;

fn main() {
    if args().len() != 2 {
        println!(
            "usage: \x1b[1m{} \x1b[35m<n: u32>\x1b[0m",
            args().next().unwrap()
        );
        return;
    }

    println!(
        "{}",
        gray_code(args().nth(1).unwrap().parse::<u32>().unwrap(),)
    );
}
