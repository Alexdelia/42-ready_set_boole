#[cfg(test)]
mod truth_table {
    use crate::ex04::truth_table::print_truth_table;

    #[test]
    fn subject() {
        print_truth_table("AB&C|");
    }

    // #[test]
    // fn all_var() {
    //     print_truth_table("AB&C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|");
    // }
}
