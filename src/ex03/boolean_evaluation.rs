macro_rules! err {
	($($arg:tt)*) => {
		eprint!("\x1b[31;1merror\x1b[0m\x1b[1m:\t");
		eprint!($($arg)*);
		eprintln!("\x1b[0m");
	};
}

pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for (i, c) in formula.chars().enumerate() {
        match c {
            '1' => stack.push(true),
            '0' => stack.push(false),
            '&' => {
                let (l, r) = pop(&mut stack, c, formula, i, true);
                stack.push(l && r);
            }
            '|' => {
                let (l, r) = pop(&mut stack, c, formula, i, true);
                stack.push(l || r);
            }
            '^' => {
                let (l, r) = pop(&mut stack, c, formula, i, true);
                stack.push(l ^ r);
            }
            '!' => {
                let (_, r) = pop(&mut stack, c, formula, i, false);
                stack.push(!r);
            }
            '>' => {
                let (l, r) = pop(&mut stack, c, formula, i, true);
                stack.push(!l || r);
            }
            '=' => {
                let (l, r) = pop(&mut stack, c, formula, i, true);
                stack.push(l == r);
            }
            _ => parse_err(
                format!("invalid char \x1b[31m\"{}\"\x1b[0m", c).as_str(),
                formula,
                i,
            ),
        }
    }

    stack.pop().unwrap()
}

fn pop(stack: &mut Vec<bool>, c: char, f: &str, i: usize, two: bool) -> (bool, bool) {
    let r = stack.pop();
    let l = if two { stack.pop() } else { Some(false) };

    if r.is_none() || l.is_none() {
        parse_err(
            format!("not enough operands for operator \x1b[35m'{}'\x1b[0m", c).as_str(),
            f,
            i,
        );
    }

    (l.unwrap(), r.unwrap())
}

fn parse_err(msg: &str, f: &str, i: usize) {
    err!("{}", msg);
    eprintln!("\t\"{}\"", f);
    eprintln!("\t{}\x1b[31;1m^\x1b[0m", " ".repeat(i + 1));
    panic!("{}", msg);
}
