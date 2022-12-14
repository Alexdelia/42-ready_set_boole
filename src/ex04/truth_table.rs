use crate::ex03::boolean_evaluation::eval_formula;

pub fn print_truth_table(formula: &str) {
    let mut var: Vec<char> = Vec::new();

    for c in 'A'..='Z' {
        if formula.contains(c) {
            var.push(c);
        }
    }

    let mut table: Vec<Vec<bool>> = Vec::new();
    let mut row: Vec<bool> = vec![false; var.len()];

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

    for v in &var {
        print!("| {} ", v);
    }
    println!("| = |");
    for _ in 0..var.len() {
        print!("|---");
    }
    println!("|---|");
    for row in table {
        for b in &row {
            print!("| {} ", *b as u8);
        }
        {
            let mut f: String = formula.to_string();
            for i in 0..var.len() {
                if row[i] {
                    f = f.replace(var[i], "1");
                } else {
                    f = f.replace(var[i], "0");
                }
            }
            println!("| {} |", eval_formula(&f) as u8);
        }
    }
}
