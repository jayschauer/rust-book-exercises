pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
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
