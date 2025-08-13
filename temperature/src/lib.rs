#[must_use]
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
#[must_use]
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
// These tests are simple and strict equality comparison works on my laptop.
// I'm not gonna get into the nuances of float comparisons for this simple exercise.
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn freezing_f_to_c_is_correct() {
        let result = fahrenheit_to_celsius(32.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn freezing_c_to_f_is_correct() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn boiling_f_to_c_is_correct() {
        let result = fahrenheit_to_celsius(212.0);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn boiling_c_to_f_is_correct() {
        let result = celsius_to_fahrenheit(100.0);
        assert_eq!(result, 212.0);
    }
}
