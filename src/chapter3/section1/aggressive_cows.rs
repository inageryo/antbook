pub fn solve(n: usize, m: usize, x_list: &mut [usize]) -> usize {
    x_list.sort();
    let mut lb = 1usize;
    let mut ub = x_list[n - 1] - x_list[0] + 1;
    while ub - lb > 1 {
        let mid = (ub + lb) / 2;
        if is_ok(mid, m, x_list) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    lb
}

fn is_ok(v: usize, m: usize, x_list: &[usize]) -> bool {
    let mut count = 1usize;
    let mut current = x_list[0];
    for x in x_list {
        if x - current >= v {
            count += 1;
            current = *x;
            if count >= m {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, 3, &mut [1, 2, 8, 4, 9], 3)]
    #[case(5, 2, &mut [1, 2, 8, 4, 9], 8)]
    #[case(5, 5, &mut [1, 2, 8, 4, 9], 1)]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] x_list: &mut [usize],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, m, x_list));
    }
}
