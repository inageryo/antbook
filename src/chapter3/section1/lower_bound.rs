pub fn solve(a_list: &[usize], k: usize) -> usize {
    a_list.partition_point(|&x| x < k)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[2, 3, 3, 5, 6], 3, 1)]
    #[case(&[2, 3, 3, 5, 6], 1, 0)]
    #[case(&[2, 3, 3, 5, 6], 6, 4)]
    #[case(&[2, 3, 3, 5, 6], 7, 5)]
    fn it_works(#[case] a_list: &[usize], #[case] k: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(a_list, k));
    }
}
