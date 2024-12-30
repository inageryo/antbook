use std::collections::VecDeque;

pub fn solve(l_list: &[usize]) -> usize {
    let mut l_list = VecDeque::from(l_list.to_vec());
    l_list.make_contiguous().sort();
    let mut ans = 0usize;
    while l_list.len() > 1 {
        let nl = l_list.pop_front().unwrap() + l_list.pop_front().unwrap();
        let idx = l_list.partition_point(|&x| x < nl);
        l_list.insert(idx, nl);
        ans += nl;
    }
    ans
}
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[1, 2, 3], 9)]
    #[case(&[8, 5, 8], 34)]
    #[case(&[1, 5, 3, 4, 2], 33)]
    #[case(&[100], 0)]
    fn it_works(#[case] l_list: &[usize], #[case] expect: usize) {
        assert_eq!(solve(l_list), expect);
    }
}
