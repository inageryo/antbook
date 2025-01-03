use crate::common::moduler::mod_pow;
use crate::common::prime::is_prime;

pub fn solve(n: usize) -> bool {
    if is_prime(n) {
        return false;
    }
    for x in 2..n {
        if mod_pow(x, n, n) != x {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(17, false)]
    #[case(561, true)]
    #[case(4, false)]
    #[case(2, false)]
    #[case(64_999, false)]
    fn it_works(#[case] n: usize, #[case] expected: bool) {
        assert_eq!(expected, solve(n));
    }
}
