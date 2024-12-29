pub fn solve(l: usize, x: Vec<usize>) -> (usize, usize) {
    let mut min_time = 0usize;
    let mut max_time = 0usize;
    for e in x.iter() {
        min_time = min_time.max(*e.min(&(l - *e)));
        max_time = max_time.max(*e.max(&(l - *e)));
    }
    (min_time, max_time)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(10, vec![2, 6, 7], (4, 8))]
    #[case(10, vec![5], (5, 5))]
    #[case(20, vec![1, 2, 7, 8, 16, 18], (8, 19))]
    fn it_works(#[case] n: usize, #[case] x: Vec<usize>, #[case] expected: (usize, usize)) {
        let result = solve(n, x);
        assert_eq!(expected, result);
    }
}
