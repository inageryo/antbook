use crate::common::bit::BIT;

pub fn solve(n: usize, a_list: &[usize]) -> usize {
    let mut bit = BIT::new(n);
    let mut ans = 0;
    for a in a_list {
        ans += (bit.sum(n) - bit.sum(*a)) as usize;
        bit.add(*a, 1);
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, &[3, 1, 4, 2], 3)]
    #[case(4, &[3, 4, 1, 2], 4)]
    #[case(4, &[1, 2, 3, 4], 0)]
    #[case(1, &[1], 0)]
    #[case(6, &[5, 3, 2, 6, 1, 4], 9)]
    fn it_works(#[case] n: usize, #[case] a_list: &[usize], #[case] expected: usize) {
        assert_eq!(expected, solve(n, a_list));
    }
}
