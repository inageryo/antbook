use crate::common::prime::is_prime;

pub fn solve(n: usize) -> bool {
    is_prime(n)
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
        let result = solve(n);
        assert_eq!(expected, result);
    }
}
