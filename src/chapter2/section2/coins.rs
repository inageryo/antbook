pub fn solve(
    c1: usize,
    c5: usize,
    c10: usize,
    c50: usize,
    c100: usize,
    c500: usize,
    a: usize,
) -> usize {
    let mut ans = 0usize;
    let mut a = a;
    for (v, c) in [
        (500, c500),
        (100, c100),
        (50, c50),
        (10, c10),
        (5, c5),
        (1, c1),
    ] {
        let ac = (a / v).min(c);
        ans += ac;
        a -= ac * v;
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 2, 1, 3, 0, 2, 620, 6)]
    #[case(3, 2, 1, 3, 1, 2, 620, 5)]
    #[case(500, 200, 100, 300, 10, 2, 500, 1)]
    fn it_works(
        #[case] c1: usize,
        #[case] c5: usize,
        #[case] c10: usize,
        #[case] c50: usize,
        #[case] c100: usize,
        #[case] c500: usize,
        #[case] a: usize,
        #[case] expected: usize,
    ) {
        let result = solve(c1, c5, c10, c50, c100, c500, a);
        assert_eq!(expected, result);
    }
}
