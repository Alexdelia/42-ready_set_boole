use ready_set_boole::ex01::multiplier::multiplier;
use std::env::args;

fn main() {
    if args().len() != 3 {
        println!(
            "usage: \x1b[1m{} \x1b[35m<a: u32> <b: u32>\x1b[0m",
            args().next().unwrap()
        );
        return;
    }

    println!(
        "{}",
        multiplier(
            args().nth(1).unwrap().parse::<u32>().unwrap(),
            args().nth(2).unwrap().parse::<u32>().unwrap()
        )
    );
}
