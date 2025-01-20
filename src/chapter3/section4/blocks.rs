use crate::common::matrix::{mod_mul, mod_pow};

const M: usize = 10usize.pow(4) + 7;
pub fn solve(n: usize) -> usize {
    let a = vec![
        vec![2, 1, 1, 0],
        vec![1, 2, 0, 1],
        vec![1, 0, 2, 1],
        vec![0, 1, 1, 2],
    ];
    let v = mod_mul(&mod_pow(&a, n, M), &[vec![1], vec![0], vec![0], vec![0]], M);
    v[0][0]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(1, 2)]
    #[case(2, 6)]
    #[case(3, 20)]
    #[case(4, 72)]
    #[case(5, 272)]
    #[case(8, 6505)]
    #[case(1_000_000_000, 3332)]
    fn it_works(#[case] n: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(n));
    }
}
