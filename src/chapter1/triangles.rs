pub fn solve(n: usize, a_list: &mut [usize]) -> usize {
    a_list.sort();
    for i in (0..n).rev() {
        if i > 1 && a_list[i - 2] + a_list[i - 1] > a_list[i] {
            return a_list[i] + a_list[i - 1] + a_list[i - 2];
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, vec![2, 3, 4, 5 ,10], 12)]
    #[case(4, vec![4, 5, 10, 20], 0)]
    #[case(3, vec![1, 99, 100], 0)]
    #[case(3, vec![1, 100, 100], 201)]
    fn it_works(#[case] n: usize, #[case] mut a_list: Vec<usize>, #[case] expected: usize) {
        let result = solve(n, &mut a_list);
        assert_eq!(expected, result);
    }
}
