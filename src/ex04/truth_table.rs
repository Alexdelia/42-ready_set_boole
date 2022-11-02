use crate::ex03::boolean_evaluation::eval_formula;

pub fn print_truth_table(formula: &str) {
    let mut var: Vec<char> = Vec::new();

    for c in 'A'..='Z' {
        if formula.contains(c) {
            var.push(c);
        }
    }

    let mut table: Vec<Vec<bool>> = Vec::new();
    let mut row: Vec<bool> = Vec::new();

    for _ in 0..var.len() {
        row.push(false);
    }

    for _ in 0..2usize.pow(var.len() as u32) {
        table.push(row.clone());
        for i in (0..row.len()).rev() {
            if !row[i] {
                row[i] = true;
                break;
            } else {
                row[i] = false;
            }
        }
    }

    for i in 0..var.len() {
        print!("| {} ", var[i]);
    }
    println!("| = |");
    for i in 0..var.len() {
        print!("|---");
    }
    println!("|---|");
    for row in table {
        for i in 0..var.len() {
            print!("| {} ", row[i] as u8);
        }
        println!("| {} |", 1);
    }
}
