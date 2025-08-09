pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_works() {
        let result = fibonacci(0);
        assert_eq!(result, 0);

        let result = fibonacci(1);
        assert_eq!(result, 1);

        let result = fibonacci(2);
        assert_eq!(result, 1);

        let result = fibonacci(3);
        assert_eq!(result, 2);

        let result = fibonacci(4);
        assert_eq!(result, 3);

        let result = fibonacci(40);
        assert_eq!(result, 102334155);
    }
}
