use crate::common::rmq::RMQ;

const INF: usize = 10usize.pow(12);
pub fn solve(n: usize, list: &[(usize, usize)]) -> usize {
    let mut dp = vec![INF; n];
    dp[0] = 0;
    let mut size = 1;
    while size < n {
        size *= 2;
    }
    let mut rmq = RMQ::new(n);
    rmq.update(0, 0);
    for e in list.iter() {
        let (s, t) = *e;
        let v = dp[t].min(rmq.query(s, t + 1, 0, 0, size) + 1);
        dp[t] = v;
        rmq.update(t, v);
    }
    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(40, &[(19, 29), (0, 9), (9, 19), (19, 29), (14, 24), (29, 39)], 4)]
    #[case(40, &[(19, 29), (0, 39), (9, 19), (19, 29), (14, 24), (29, 39)], 1)]
    #[case(40, &[(19, 29), (0, 9), (9, 19), (0, 24), (19, 29), (14, 24), (29, 39)], 3)]
    #[case(2, &[(0, 1)], 1)]
    fn it_works(#[case] n: usize, #[case] list: &[(usize, usize)], #[case] expected: usize) {
        assert_eq!(expected, solve(n, list));
    }
}
