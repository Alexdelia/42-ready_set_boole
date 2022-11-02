use ready_set_boole::ex04::truth_table::print_truth_table;
use std::env::args;

fn main() {
    if args().len() != 2 {
        println!(
            "usage: \x1b[1m{} \x1b[35m<formula>\x1b[0m",
            args().next().unwrap()
        );
        return;
    }

    print_truth_table(args().nth(1).unwrap().as_str());
}
