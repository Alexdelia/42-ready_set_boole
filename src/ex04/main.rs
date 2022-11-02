#[cfg(test)]
mod truth_table {
    use crate::ex04::truth_table::print_truth_table;

    #[test]
    fn basic() {
        print_truth_table("AB&");
        print_truth_table("AB|C&");
    }
}
