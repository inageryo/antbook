pub fn solve(n: usize, m: usize, md: usize) -> usize {
    let mut dp = vec![vec![0; n + 1]; m + 1];
    dp[0][0] = 1;
    for i in 1..=m {
        for j in 0..=n {
            dp[i][j] = if j >= i {
                (dp[i - 1][j] + dp[i][j - i]) % md
            } else {
                dp[i - 1][j]
            }
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, 3, 10_000, 4)]
    #[case(4, 4, 10_000, 5)]
    #[case(5, 5, 10_000, 7)]
    #[case(10, 10, 10_000, 42)]
    #[case(1_000, 1_000, 10_000, 7991)]
    fn it_works(#[case] n: usize, #[case] m: usize, #[case] md: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(n, m, md));
    }
}
