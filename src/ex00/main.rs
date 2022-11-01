#[cfg(test)]
mod adder {
    use crate::ex00::adder::adder;

    #[test]
    fn basic() {
        assert_eq!(adder(1, 3), 4);
    }

    #[test]
    fn two_zero() {
        assert_eq!(adder(0, 0), 0);
    }

    #[test]
    fn one_zero() {
        assert_eq!(adder(0, 42), 42);
    }

    #[test]
    fn one_zero_reverse() {
        assert_eq!(adder(42, 0), 42);
    }

    #[test]
    fn two_max() {
        assert_eq!(
            adder(u32::max_value(), u32::max_value()),
            u32::max_value() - 1
        );
    }

    #[test]
    fn one_max() {
        assert_eq!(adder(u32::max_value(), 42), u32::max_value() + 42);
    }

    #[test]
    fn one_max_reverse() {
        assert_eq!(adder(42, u32::max_value()), u32::max_value() + 42);
    }
}
