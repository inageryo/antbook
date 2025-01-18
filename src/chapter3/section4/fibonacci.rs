use crate::common::matrix::{mod_mul, mod_pow};
const M: usize = 10usize.pow(4);

pub fn solve(n: usize) -> usize {
    let a = vec![vec![1, 1], vec![1, 0]];
    let v = mod_mul(&mod_pow(&a, n, M), &[vec![1], vec![0]], M);
    v[1][0]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(10, 55)]
    #[case(20, 6765)]
    #[case(30, 2040)]
    fn it_works(#[case] n: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(n));
    }
}
