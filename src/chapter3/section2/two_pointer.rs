pub fn solve(n: usize, s: usize, a_list: &[usize]) -> usize {
    let mut ans = usize::MAX;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut current = a_list[0];
    loop {
        if current < s {
            p2 += 1;
            if p2 >= n {
                break;
            }
            current += a_list[p2];
        } else {
            ans = ans.min(p2 - p1 + 1);
            current -= a_list[p1];
            p1 += 1;
            if p1 > p2 {
                return ans;
            }
        }
    }
    if ans == usize::MAX {
        return 0;
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(10, 15, &[5, 1, 3, 5, 10, 7, 4, 9, 2, 8], 2)]
    #[case(5, 11, &[1, 2, 3, 4, 5], 3)]
    #[case(5, 20, &[1, 2, 3, 4, 5], 0)]
    #[case(5, 1, &[1, 2, 3, 4, 5], 1)]
    #[case(5, 5, &[1, 2, 3, 4, 5], 1)]
    #[case(5, 6, &[1, 2, 3, 4, 5], 2)]
    fn it_works(
        #[case] n: usize,
        #[case] s: usize,
        #[case] a_list: &[usize],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, s, a_list));
    }
}
