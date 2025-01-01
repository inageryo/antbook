use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve(l_list: &[usize]) -> usize {
    let mut pq = BinaryHeap::from_iter(l_list.iter().map(|&e| Reverse(e)));
    let mut ans = 0usize;
    while pq.len() > 1 {
        let nl = pq.pop().unwrap().0 + pq.pop().unwrap().0;
        pq.push(Reverse(nl));
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
    fn it_works(#[case] l_list: &[usize], #[case] expected: usize) {
        assert_eq!(expected, solve(l_list));
    }
}
