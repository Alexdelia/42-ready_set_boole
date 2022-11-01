#[cfg(test)]
mod gray_code {
    use crate::ex03::boolean_evaluation::eval_formula;

    #[test]
    fn bool() {
        assert_eq!(eval_formula("1"), true);
        assert_eq!(eval_formula("0"), false);
    }

    #[test]
    fn conjunction() {
        assert_eq!(eval_formula("11&"), true);
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("01&"), false);
        assert_eq!(eval_formula("00&"), false);
    }

    #[test]
    fn disjunction() {
        assert_eq!(eval_formula("11|"), true);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("01|"), true);
        assert_eq!(eval_formula("00|"), false);
    }

    #[test]
    fn exclusive_disjunction() {
        assert_eq!(eval_formula("11^"), false);
        assert_eq!(eval_formula("10^"), true);
        assert_eq!(eval_formula("01^"), true);
        assert_eq!(eval_formula("00^"), false);
    }

    #[test]
    fn negation() {
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("0!"), true);
    }

    #[test]
    fn material_condition() {
        assert_eq!(eval_formula("11>"), true, "11>");
        assert_eq!(eval_formula("10>"), false, "10>");
        assert_eq!(eval_formula("01>"), true, "01>");
        assert_eq!(eval_formula("00>"), true, "00>");
    }

    #[test]
    fn logical_equivalence() {
        assert_eq!(eval_formula("11="), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("01="), false);
        assert_eq!(eval_formula("00="), true);
    }

    #[test]
    fn multiple() {
        assert_eq!(eval_formula("1!1&"), false);
        assert_eq!(eval_formula("1!1&1|"), true);
        assert_eq!(eval_formula("1!1&1|0^"), true);
        assert_eq!(eval_formula("1!1&1|0^1="), true);
    }

    #[test]
    fn subject() {
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    #[should_panic]
    fn invalid_char() {
        eval_formula("1a");
    }

    #[test]
    #[should_panic]
    fn invalid_char_two() {
        eval_formula("12&");
    }

    #[test]
    #[should_panic]
    fn not_enough_operands() {
        eval_formula("1&");
    }

    #[test]
    #[should_panic]
    fn not_enough_operands_empty() {
        eval_formula("!");
    }

    #[test]
    #[should_panic]
    fn not_enough_operands_complex() {
        eval_formula("10101|1&&&&&&01|=11&");
    }
}
