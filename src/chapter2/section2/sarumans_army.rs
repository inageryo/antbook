pub fn solve(r: usize, x_list: &[usize]) -> usize {
    let mut ans = 0;
    let mut index = 0;
    while index < x_list.len() {
        let current = x_list[index];
        let ni = x_list.partition_point(|&x| x <= current + r) - 1;
        ans += 1;
        let nc = x_list[ni];
        let ni = x_list.partition_point(|&x| x <= nc + r);
        index = ni;
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::normal(10, &[1, 7, 15, 20, 30, 50], 3)]
    #[case::normal(10, &[11, 21], 1)]
    #[case::normal(10, &[11, 22], 2)]
    #[case::normal(10, &[21, 31, 41], 1)]
    #[case::normal(10, &[20, 40, 60], 3)]
    fn it_works(#[case] r: usize, #[case] x_list: &[usize], #[case] expected: usize) {
        assert_eq!(expected, solve(r, x_list));
    }
}
