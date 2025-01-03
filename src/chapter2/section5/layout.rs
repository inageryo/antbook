const INF: isize = 10isize.pow(12);
pub fn solve(
    n: usize,
    l_list: &[(usize, usize, usize)],
    d_list: &[(usize, usize, usize)],
) -> isize {
    let mut d = vec![0; n];
    let mut updated = false;
    bf(n, l_list, d_list, &mut d, &mut updated);
    if updated {
        return -1;
    }
    d = vec![INF; n];
    d[0] = 0;
    bf(n, l_list, d_list, &mut d, &mut updated);
    if d[n - 1] == INF {
        return -2;
    }
    d[n - 1]
}

fn update(x: &mut isize, y: &isize, updated: &mut bool) {
    if *x > *y {
        *updated = true;
        *x = *y;
    }
}

fn bf(
    n: usize,
    l_list: &[(usize, usize, usize)],
    d_list: &[(usize, usize, usize)],
    d: &mut [isize],
    updated: &mut bool,
) {
    for _ in 0..n {
        *updated = false;
        for i in 0..n - 1 {
            if d[i + 1] < INF {
                let tmp = d[i + 1];
                update(&mut d[i], &tmp, updated);
            }
        }
        for (al, bl, dl) in l_list.iter() {
            if d[*al] < INF {
                let tmp = d[*al] + *dl as isize;
                update(&mut d[*bl], &tmp, updated);
            }
        }
        for (ad, bd, dd) in d_list.iter() {
            if d[*bd] < INF {
                let tmp = d[*bd] - *dd as isize;
                update(&mut d[*ad], &tmp, updated);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, &[(0, 2, 10), (1, 3, 20)], &[(1, 2, 3)], 27)]
    #[case(2, &[(0, 1, 10)], &[], 10)]
    #[case(3, &[(0, 1, 10)], &[(1, 2, 10)], -2)]
    #[case(3, &[(0, 1, 10), (0, 2, 10)], &[], 10)]
    #[case(3, &[(0, 2, 9)], &[(0, 1, 10)], -1)]
    fn it_works(
        #[case] n: usize,
        #[case] l_list: &[(usize, usize, usize)],
        #[case] d_list: &[(usize, usize, usize)],
        #[case] expected: isize,
    ) {
        assert_eq!(expected, solve(n, l_list, d_list));
    }
}
