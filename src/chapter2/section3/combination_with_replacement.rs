pub fn solve(n: usize, m: usize, a_list: &[usize], md: usize) -> usize {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for e in dp.iter_mut() {
        e[0] = 1;
    }
    let mut dp_accumulated_sum = vec![vec![0; m + 1]; n + 1];
    for e in dp_accumulated_sum.iter_mut() {
        e[0] = 1;
    }
    for e in dp_accumulated_sum[0].iter_mut() {
        *e = 1;
    }
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if j > a_list[i - 1] {
                dp_accumulated_sum[i - 1][j] - dp_accumulated_sum[i - 1][j - a_list[i - 1] - 1] % md
            } else {
                dp_accumulated_sum[i - 1][j]
            };
            dp_accumulated_sum[i][j] = (dp_accumulated_sum[i][j - 1] + dp[i][j]) % md;
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 3, &[1, 2, 3], 10_000, 6)]
    #[case(1, 1, &[10], 10_000, 1)]
    #[case(4, 3, &[3, 4, 3, 2], 10_000, 19)]
    #[case(4, 3, &[1, 2, 3, 4], 10_000, 15)]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] a_list: &[usize],
        #[case] md: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, m, a_list, md));
    }
}
