#[must_use]
pub fn fibonacci(n: u128) -> u128 {
    fibonacci_fast_double(n).0
}

#[allow(clippy::similar_names)]
fn fibonacci_fast_double(n: u128) -> (u128, u128) {
    // Based on https://cp-algorithms.com/algebra/fibonacci-numbers.html#fast-doubling-method
    // Good proof here: https://medium.com/@ani1998ket/deriving-the-fast-fibonacci-identities-without-matrices-o-log-n-4cd9ce69d9d4

    if n == 0 {
        return (0, 1); // return F(n), F(n+1). Base case for n = 0 is (0, 1)
    }

    // floor divide n by 2
    // n = 2k or 2k + 1 depending on whether n is even or odd
    let k = n / 2;
    let (fib_k, fib_k_plus_1) = fibonacci_fast_double(k);

    // F(2k) = F(k) * (2*F(k+1) - F(k))
    let fib_2k = fib_k * (2 * fib_k_plus_1 - fib_k);
    // F(2k+1) = F(k+1)^2 + F(k)^2
    let fib_2k_plus_1 = fib_k_plus_1 * fib_k_plus_1 + fib_k * fib_k;

    // Now we want to return F(n) and F(n+1)
    if n % 2 == 0 {
        // if n is even, n = 2k, so return F(2k) and F(2k+1)
        (fib_2k, fib_2k_plus_1)
    } else {
        // if n is odd, n = 2k + 1, so return F(2k+1) and F(2k+2) = F(2k) + F(2k+1)
        (fib_2k_plus_1, fib_2k + fib_2k_plus_1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_works() {
        // Pulled example values from https://oeis.org/A000045

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

        let result = fibonacci(39);
        assert_eq!(result, 63_245_986);

        let result = fibonacci(40);
        assert_eq!(result, 102_334_155);

        let result = fibonacci(185);
        assert_eq!(result, 205_697_230_343_233_228_174_223_751_303_346_572_685);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn fibonacci_overflow() {
        // This will panic in debug builds due to overflow
        let _ = fibonacci(186);
    }
}
