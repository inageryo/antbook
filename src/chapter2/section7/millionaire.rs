pub fn solve(m: usize, p: f64, x: usize) -> String {
    let mut dp = vec![vec![0.; (1 << m) + 1]; m + 1];
    let n = 1 << m;
    dp[0][n] = 1.;
    for r in 0..m {
        for i in 0..=n {
            let limit = i.min(n - i);
            let mut t: f64 = 0.;
            for j in 0..=limit {
                t = t.max(p * dp[r][i + j] + (1. - p) * dp[r][i - j]);
            }
            dp[r + 1][i] = t;
        }
    }
    let index = x * n / 1_000_000;
    format!("{:.6}", dp[m][index])
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 0.5, 500000, "0.500000")]
    #[case(3, 0.75, 600000, "0.843750")]
    #[case(3, 0.15, 1000000, "1.000000")]
    #[case(10, 0.15, 900000, "0.389743")]
    fn it_works(#[case] m: usize, #[case] p: f64, #[case] x: usize, #[case] expected: String) {
        assert_eq!(expected, solve(m, p, x));
    }
}
