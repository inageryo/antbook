use crate::common::matrix::{mod_mul, mod_pow};

pub fn solve(n: usize, k: usize, m: usize, a: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut a1 = vec![vec![0; 2 * n]; 2 * n];
    let mut b = vec![vec![0; n]; 2 * n];
    for i in 0..n {
        for j in 0..n {
            a1[i][j] = a[i][j];
        }
        a1[i + n][i] = 1;
        a1[i + n][i + n] = 1;
        b[i][i] = 1;
    }
    let v = mod_mul(&mod_pow(&a1, k + 1, m), &b, m);
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                ans[i][j] = (v[i + n][j] + m - 1) % m;
            } else {
                ans[i][j] = v[i + n][j] % m;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(2, 2, 4, &[vec![0, 1], vec![1, 1]], vec![vec![1, 2], vec![2, 3]])]
    #[case(2, 3, 10, &[vec![0, 1], vec![1, 1]], vec![vec![2, 4], vec![4, 6]])]
    #[case(2, 3, 4, &[vec![0, 1], vec![1, 1]], vec![vec![2, 0], vec![0, 2]])]
    #[case(1, 1, 4, &[vec![1]], vec![vec![1]])]
    #[case(1, 3, 4, &[vec![1]], vec![vec![3]])]
    #[case(2, 3, 10, &[vec![1, 1], vec![1, 1]], vec![vec![7, 7], vec![7, 7]])]
    fn it_works(
        #[case] n: usize,
        #[case] k: usize,
        #[case] m: usize,
        #[case] a: &[Vec<usize>],
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert_eq!(expected, solve(n, k, m, a));
    }
}
