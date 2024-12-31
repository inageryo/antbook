pub fn solve(n: usize, a_list: &[usize]) -> usize {
    // binary search
    let inf = 10usize.pow(12);
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    for e in dp.iter_mut() {
        e[0] = 0;
    }
    for i in 1..=n {
        for j in 1..=n {
            dp[i][j] = if dp[i - 1][j - 1] < a_list[i - 1] {
                dp[i - 1][j].min(a_list[i - 1])
            } else {
                dp[i - 1][j]
            }
        }
    }
    for j in (0..=n).rev() {
        if dp[n][j] < inf {
            return j;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, vec![4, 2, 3, 1, 5], 3)]
    #[case(5, vec![1, 2, 3, 1, 5], 4)]
    #[case(5, vec![1, 2, 3, 4, 5], 5)]
    #[case(5, vec![5, 4, 3, 2, 1], 1)]
    #[case(1, vec![5], 1)]
    #[case(2, vec![5, 5], 1)]
    fn it_works(#[case] n: usize, #[case] a_list: Vec<usize>, #[case] expected: usize) {
        assert_eq!(expected, solve(n, &a_list));
    }
}
