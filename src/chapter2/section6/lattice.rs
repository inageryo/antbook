use crate::common::gcd::gcd;

pub fn solve(p1: (isize, isize), p2: (isize, isize)) -> usize {
    if p1 == p2 {
        return 0;
    }
    gcd((p1.0 - p2.0).unsigned_abs(), (p1.1 - p2.1).unsigned_abs()) - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((1, 11), (5, 3), 3)]
    #[case((1, 11), (1, 11), 0)]
    #[case((0, 11), (1, 11), 0)]
    #[case((0, 11), (10, 11), 9)]
    #[case((0, 5), (0, 10), 4)]
    #[case((1, 11), (6, 3), 0)]
    fn it_works(#[case] p1: (isize, isize), #[case] p2: (isize, isize), #[case] expected: usize) {
        assert_eq!(expected, solve(p1, p2));
    }
}
