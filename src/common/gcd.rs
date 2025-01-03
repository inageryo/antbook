pub fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        return n;
    }
    gcd(m, n % m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 2, 2)]
    #[case(2, 5, 1)]
    #[case(1, 5, 1)]
    #[case(26, 32, 2)]
    #[case(29, 32, 1)]
    #[case(28, 32, 4)]
    fn gcd_works(#[case] n: usize, #[case] m: usize, #[case] expected: usize) {
        assert_eq!(expected, gcd(n, m));
    }
}
