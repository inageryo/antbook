pub fn solve(n: usize, l: &[(usize, usize)], w: usize) -> usize {
    let inf = 10usize.pow(12);
    let max_v = l.iter().map(|&(_, v)| v).sum::<usize>();
    let mut dp = vec![vec![inf; max_v + 2]; n + 1];
    for e in dp.iter_mut().take(n) {
        e[0] = 0;
    }
    for i in 1..=n {
        for j in (1..=max_v).rev() {
            dp[i][j] = if j >= l[i - 1].1 {
                dp[i - 1][j]
                    .min(dp[i][j + 1])
                    .min(dp[i - 1][j - l[i - 1].1] + l[i - 1].0)
            } else {
                dp[i - 1][j].min(dp[i][j + 1])
            }
        }
    }
    for (i, e) in dp[n].iter().rev().enumerate() {
        if *e <= w {
            return max_v + 1 - i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, vec![(2, 3), (1, 2), (3, 4), (2, 2)], 5, 7)]
    #[case(4, vec![(1, 3), (1, 2), (3, 4), (2, 2)], 5, 9)]
    #[case(4, vec![(2, 3), (1, 2), (3, 4), (2, 2)], 1, 2)]
    #[case(4, vec![(2, 3), (1, 2), (3, 4), (2, 2)], 1_000_000_000, 11)]
    #[case(1, vec![(2, 3)], 2, 3)]
    #[case(1, vec![(2, 3)], 1, 0)]
    fn it_works(
        #[case] n: usize,
        #[case] l: Vec<(usize, usize)>,
        #[case] w: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, &l, w))
    }
}
