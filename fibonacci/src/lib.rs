pub fn fibonacci(n: u128) -> u128 {
    fibonacci_fast_double(n).0
}

fn fibonacci_fast_double(n: u128) -> (u128, u128) {
    // Based on https://cp-algorithms.com/algebra/fibonacci-numbers.html#fast-doubling-method
    // Good proof here: https://medium.com/@ani1998ket/deriving-the-fast-fibonacci-identities-without-matrices-o-log-n-4cd9ce69d9d4

    if n == 0 {
        return (0, 1); // return F(n), F(n+1). Base case for n = 0 => (0, 1)
    }

    // floor divide n by 2
    // n = 2k or 2k + 1 depending on whether n is even or odd
    let (fk, fk1) = fibonacci_fast_double(n / 2);

    let f2k = fk * (2 * fk1 - fk); // F(2k) = F(k) * (2*F(k+1) - F(k))
    let f2k1 = fk1 * fk1 + fk * fk; // F(2k+1) = F(k+1)^2 + F(k)^2

    if n % 2 == 0 {
        // if n is even, we calculated the right thing with our recursive call: F(2k) and F(2k+1)
        (f2k, f2k1)
    } else {
        // if n is odd (n = 2k + 1), return (F(2k+1), F(2k+2)) = (f2k1, f2k + f2k1)
        (f2k1, f2k + f2k1)
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
        assert_eq!(result, 63245986);

        let result = fibonacci(40);
        assert_eq!(result, 102334155);

        let result = fibonacci(185);
        assert_eq!(result, 205697230343233228174223751303346572685);
    }

    #[test]
    #[should_panic]
    fn fibonacci_overflow() {
        // This will panic in debug builds due to overflow
        fibonacci(186);
    }
}
