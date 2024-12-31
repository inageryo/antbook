pub fn solve(n: usize, m: usize, s: &[char], t: &[char]) -> usize {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if s[i - 1] == t[j - 1] {
                dp[i][j - 1].max(dp[i - 1][j]).max(dp[i - 1][j - 1] + 1)
            } else {
                dp[i][j - 1].max(dp[i - 1][j])
            }
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, 4, vec!['a', 'b', 'c', 'd'], vec!['b', 'e', 'c', 'd'], 3)]
    #[case(4, 4, vec!['a', 'b', 'c', 'd'], vec!['b', 'c', 'c', 'd'], 3)]
    #[case(4, 1, vec!['a', 'b', 'c', 'd'], vec!['a'], 1)]
    #[case(4, 5, vec!['a', 'b', 'c', 'd'], vec!['e', 'b', 'a', 'd', 'c'], 2)]
    #[case(4, 4, vec!['a', 'b', 'c', 'd'], vec!['e', 'f', 'g', 'h'], 0)]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] s: Vec<char>,
        #[case] t: Vec<char>,
        #[case] expected: usize,
    ) {
        assert_eq!(solve(n, m, &s, &t), expected);
    }
}
