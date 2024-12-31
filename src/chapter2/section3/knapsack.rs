pub fn solve(n: usize, l: &[(usize, usize)], w: usize) -> usize {
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        for j in 0..w + 1 {
            dp[i + 1][j] = if j >= l[i].0 {
                dp[i][j].max(dp[i][j - l[i].0] + l[i].1)
            } else {
                dp[i][j]
            };
        }
    }
    dp[n][w]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, vec![(2, 3), (1, 2), (3, 4), (2, 2)], 5, 7)]
    #[case(4, vec![(1, 3), (1, 2), (3, 4), (2, 2)], 5, 9)]
    #[case(4, vec![(2, 3), (1, 2), (3, 4), (2, 2)], 1, 2)]
    fn it_works(
        #[case] n: usize,
        #[case] l: Vec<(usize, usize)>,
        #[case] w: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, &l, w))
    }
}
