use itertools::Itertools;

pub fn solve(n: usize, s: &[usize], t: &[usize]) -> usize {
    let job_list = (0..n)
        .map(|i| (s[i], t[i]))
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .collect::<Vec<_>>();
    let mut ans = 0usize;
    let mut current = 0usize;
    for (s, t) in job_list.iter() {
        if *s > current {
            ans += 1;
            current = *t;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, vec![1, 2, 4, 6, 8], vec![3, 5, 7, 9, 10], 3)]
    #[case(2, vec![1, 2], vec![2, 4], 1)]
    #[case(3, vec![1, 5, 8], vec![6, 9, 20], 2)]
    #[case(11, vec![1, 2, 2, 2, 4, 6, 8, 10, 10, 10, 12], vec![3, 5, 5, 5, 7, 9, 11, 13, 13, 13, 14], 4)]
    fn it_works(
        #[case] n: usize,
        #[case] s: Vec<usize>,
        #[case] t: Vec<usize>,
        #[case] expected: usize,
    ) {
        let result = solve(n, &s, &t);
        assert_eq!(expected, result);
    }
}
