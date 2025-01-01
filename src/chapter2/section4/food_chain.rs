use crate::common::union_find::UnionFind;

pub fn solve(n: usize, info_list: &[(usize, isize, isize)]) -> usize {
    let mut uf = UnionFind::new(n * 3);
    let mut ans = 0usize;
    for (t, x, y) in info_list {
        if *x <= 0 || n as isize <= *x || *y <= 0 || n as isize <= *y {
            ans += 1;
            continue;
        }
        let x = (x - 1) as usize;
        let y = (y - 1) as usize;
        if *t == 1 {
            if uf.find(x) == uf.find(y + n) || uf.find(x) == uf.find(y + 2 * n) {
                ans += 1;
            } else {
                uf.union(x, y);
                uf.union(x + n, y + n);
                uf.union(x + 2 * n, y + 2 * n);
            }
        } else if uf.find(x) == uf.find(y) || uf.find(x) == uf.find(y + 2 * n) {
            ans += 1;
        } else {
            uf.union(x, y + n);
            uf.union(x + n, y + 2 * n);
            uf.union(x + 2 * n, y);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(100, &[(1, 101, 1), (2, 1, 2), (2, 2, 3), (2, 3, 3), (1, 1, 3), (2, 3, 1), (1, 5, 5)], 3)]
    #[case(100, &[(1, 101, 1), (2, -1, -2), (2, 2, 2)], 3)]
    #[case(100, &[(1, 2, 1), (1, 4, 5), (2, 1, 4), (2, 4, 1), (2, 3, 5), (2, 3, 1), (2, 4, 3), (1, 2, 3)], 3)]
    fn it_works(
        #[case] n: usize,
        #[case] info_list: &[(usize, isize, isize)],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, info_list));
    }
}
