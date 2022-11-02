use ready_set_boole::ex03::boolean_evaluation::eval_formula;
use std::env::args;

fn main() {
    if args().len() != 2 {
        println!(
            "usage: \x1b[1m{} \x1b[35m<formula>\x1b[0m",
            args().next().unwrap()
        );
        return;
    }

    println!("{}", eval_formula(args().nth(1).unwrap().as_str()));
}
