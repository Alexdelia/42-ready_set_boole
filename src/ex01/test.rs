#[cfg(test)]
mod multiplier {
    use crate::ex01::multiplier::multiplier;

    #[test]
    fn basic() {
        assert_eq!(multiplier(2, 3), 6);
    }

    #[test]
    fn big() {
        assert_eq!(multiplier(42, 420000), 17640000);
    }

    #[test]
    fn two_zero() {
        assert_eq!(multiplier(0, 0), 0);
    }

    #[test]
    fn one_zero() {
        assert_eq!(multiplier(0, 42), 0);
    }

    #[test]
    fn one_zero_reverse() {
        assert_eq!(multiplier(42, 0), 0);
    }

    #[test]
    fn two_max() {
        assert_eq!(
            multiplier(u32::max_value(), u32::max_value()),
            u32::max_value() * u32::max_value()
        );
    }

    #[test]
    fn one_max() {
        assert_eq!(multiplier(u32::max_value(), 42), u32::max_value() * 42);
    }

    #[test]
    fn one_max_reverse() {
        assert_eq!(multiplier(42, u32::max_value()), 42 * u32::max_value());
    }
}
