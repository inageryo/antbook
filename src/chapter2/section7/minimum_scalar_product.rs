pub fn solve(n: usize, v1: &mut [isize], v2: &mut [isize]) -> isize {
    v1.sort();
    v2.sort();
    v2.reverse();
    let mut ans = 0;
    for i in 0..n {
        ans += v1[i] * v2[i];
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, &mut [-5, 1, 3], &mut [-2, 4, 1], -25)]
    #[case(5, &mut [1, 2, 3, 4, 5], &mut [1, 0, 1, 0, 1], 6)]
    #[case(1, &mut [100_000], &mut [100_000], 10_000_000_000)]
    #[case(4, &mut [1, 2, 5, 10], &mut [1, 7, 8, 10], 71)]
    #[case(4, &mut [1, 2, 3, 10], &mut [1, 3, 9, 10], 47)]
    fn it_works(
        #[case] n: usize,
        #[case] v1: &mut [isize],
        #[case] v2: &mut [isize],
        #[case] expected: isize,
    ) {
        assert_eq!(expected, solve(n, v1, v2));
    }
}
