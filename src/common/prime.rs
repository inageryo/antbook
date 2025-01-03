pub fn is_prime(n: usize) -> bool {
    for i in 2..=(n as f64).sqrt() as usize {
        if n % i == 0 {
            return false;
        }
    }
    n != 0 && n != 1
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, false)]
    #[case(1, false)]
    #[case(2, true)]
    #[case(3, true)]
    #[case(4, false)]
    #[case(5, true)]
    #[case(9, false)]
    #[case(10, false)]
    #[case(11, true)]
    fn is_prime_works(#[case] n: usize, #[case] expected: bool) {
        let result = is_prime(n);
        assert_eq!(expected, result);
    }
}
