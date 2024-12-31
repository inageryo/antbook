pub fn solve(n: usize, a_list: &[isize], m_list: &[isize], k: usize) -> bool {
    let mut dp = vec![vec![-1; k + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=k {
            dp[i][j] = if dp[i - 1][j] >= 0 {
                m_list[i - 1]
            } else if j < a_list[i - 1] as usize || dp[i][j - a_list[i - 1] as usize] < 0 {
                -1
            } else {
                dp[i][j - a_list[i - 1] as usize] - 1
            }
        }
    }
    dp[n][k] >= 0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, vec![3, 5, 8], vec![3, 2, 2], 17, true)]
    #[case(3, vec![3, 5, 8], vec![3, 2, 2], 2, false)]
    #[case(3, vec![3, 5, 8], vec![3, 2, 2], 100_000, false)]
    #[case(3, vec![3, 5, 8], vec![3, 2, 2], 35, true)]
    fn it_works(
        #[case] n: usize,
        #[case] a_list: Vec<isize>,
        #[case] m_list: Vec<isize>,
        #[case] k: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, solve(n, &a_list, &m_list, k));
    }
}
