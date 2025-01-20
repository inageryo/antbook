use crate::common::matrix::{mod_mul, mod_pow, transpose};

const M: usize = 10usize.pow(4) + 7;

pub fn solve(n: usize, k: usize, graph: &[Vec<usize>]) -> usize {
    let a = transpose(graph);
    let v = mod_mul(&mod_pow(&a, k, M), &vec![vec![1]; n], M);
    v.iter().map(|x| x[0]).sum::<usize>() % M
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(4, 1, &[vec![0, 1, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 0, 0]], 5)]
    #[case(4, 2, &[vec![0, 1, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 0, 0]], 6)]
    #[case(4, 1, &[vec![0, 1, 1, 1], vec![1, 0, 1, 1], vec![1, 1, 0, 1], vec![1, 1, 1, 0]], 12)]
    #[case(4, 2, &[vec![0, 1, 1, 1], vec![1, 0, 1, 1], vec![1, 1, 0, 1], vec![1, 1, 1, 0]], 36)]
    #[case(4, 4, &[vec![0, 1, 1, 1], vec![1, 0, 1, 1], vec![1, 1, 0, 1], vec![1, 1, 1, 0]], 324)]
    #[case(4, 1_000_000_000, &[vec![0, 1, 1, 1], vec![1, 0, 1, 1], vec![1, 1, 0, 1], vec![1, 1, 1, 0]], 9906)]
    fn it_works(
        #[case] n: usize,
        #[case] k: usize,
        #[case] graph: &[Vec<usize>],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, k, graph));
    }
}
