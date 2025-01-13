const INF: usize = 10usize.pow(12);
pub fn solve(n: usize, d: &[Vec<usize>]) -> usize {
    let size = 1 << n;
    let mut dp = vec![vec![INF; n]; size];
    dp[size - 1][0] = 0;
    for s in (0..size - 1).rev() {
        for (v, dv) in d.iter().enumerate() {
            for u in 0..n {
                if s & (1 << u) == 0 {
                    dp[s][v] = dp[s][v].min(dp[s | (1 << u)][u] + dv[u]);
                }
            }
        }
    }
    dp[0][0]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, &[vec![INF, 3, INF, 4, INF], vec![INF, INF, 5, INF, INF], vec![4, INF, INF, 5, INF], vec![INF, INF, INF, INF, 3], vec![7, 6, INF, INF, INF]], 22)]
    #[case(2, &[vec![INF, 5], vec![5, INF]], 10)]
    #[case(3, &[vec![INF, 1, 4], vec![5, INF, 2], vec![3, 6, INF]], 6)]
    fn it_works(#[case] n: usize, #[case] d: &[Vec<usize>], #[case] expected: usize) {
        assert_eq!(expected, solve(n, d));
    }
}
