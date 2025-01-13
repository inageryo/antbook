const INF: f64 = 1e12;
pub fn solve(
    n: usize,
    m: usize,
    a: usize,
    b: usize,
    t_list: &[f64],
    d_list: &[Vec<f64>],
) -> Option<f64> {
    let size = 1 << n;
    let mut dp = vec![vec![INF; m]; size];
    dp[0][a] = 0.;
    for s in 0..size - 1 {
        for (v, dv) in d_list.iter().enumerate() {
            for (u, dvu) in dv.iter().enumerate() {
                if *dvu > 0. {
                    for t in 0..n {
                        if s & (1 << t) == 0 {
                            dp[s | (1 << t)][u] =
                                dp[s | (1 << t)][u].min(dp[s][v] + dvu / t_list[t]);
                        }
                    }
                }
            }
        }
    }
    let mut ans = INF;
    for dpi in dp.iter() {
        ans = ans.min(dpi[b]);
    }
    if ans == INF {
        None
    } else {
        Some(ans)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(2, 4, 1, 0, &[3., 1.], &[vec![-1., -1., 3., 2.], vec![-1., -1., 3., 5.], vec![3., 3., -1., -1.], vec![2., 5., -1., -1.]], Some(3.667))]
    #[case(2, 4, 0, 3, &[3., 1.], &[vec![-1., -1., 3., 2.], vec![-1., -1., 3., 5.], vec![3., 3., -1., -1.], vec![2., 5., -1., -1.]], Some(0.667))]
    #[case(1, 4, 1, 0, &[3.], &[vec![-1., -1., 3., 2.], vec![-1., -1., 3., 5.], vec![3., 3., -1., -1.], vec![2., 5., -1., -1.]], None)]
    #[case(2, 4, 0, 3, &[3., 1.], &[vec![-1., 3., 5., -1.], vec![3., -1., 4., -1.], vec![5., 4., -1., -1.], vec![-1., -1., -1., -1.]], None)]
    #[case(1, 2, 1, 0, &[3.], &[vec![-1., 3.], vec![3., -1.]], Some(1.))]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] a: usize,
        #[case] b: usize,
        #[case] t_list: &[f64],
        #[case] d_list: &[Vec<f64>],
        #[case] expected: Option<f64>,
    ) {
        let actual = solve(n, m, a, b, t_list, d_list);
        if let Some(e) = expected {
            assert!((e - actual.unwrap()).abs() < 1e-3)
        } else {
            assert!(expected.is_none());
            assert!(actual.is_none());
        }
    }
}
