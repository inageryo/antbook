use crate::common::matrix::{mod_mul_isize, mod_pow_isize};

const M: isize = 10isize.pow(3);
pub fn solve(n: usize) -> String {
    let a = vec![vec![6, -4], vec![1, 0]];
    let v = mod_mul_isize(&mod_pow_isize(&a, n, M), &[vec![3], vec![1]], M);
    format!("{:03}", ((v[1][0] + M) * 2 - 1) % M)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, "005")]
    #[case(2, "027")]
    #[case(5, "935")]
    #[case(10, "047")]
    #[case(8, "991")]
    #[case(100, "751")]
    #[case(1_234_567, "903")]
    #[case(1_000_000_000, "751")]
    fn it_works(#[case] n: usize, #[case] expected: String) {
        assert_eq!(expected, solve(n));
    }
}
