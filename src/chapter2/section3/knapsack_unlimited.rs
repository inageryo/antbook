pub fn solve(n: usize, l: &[(usize, usize)], w: usize) -> usize {
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=w {
            dp[i][j] = if j >= l[i - 1].0 {
                dp[i - 1][j].max(dp[i][j - l[i - 1].0] + l[i - 1].1)
            } else {
                dp[i - 1][j]
            }
        }
    }
    dp[n][w]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, vec![(3, 4), (4, 5), (2, 3)], 7, 10)]
    #[case(3, vec![(3, 4), (4, 7), (2, 3)], 10, 17)]
    #[case(2, vec![(10, 4), (10, 7)], 10, 7)]
    #[case(2, vec![(10, 4), (10, 7)], 8, 0)]
    fn it_works(
        #[case] n: usize,
        #[case] l: Vec<(usize, usize)>,
        #[case] w: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, &l, w))
    }
}
