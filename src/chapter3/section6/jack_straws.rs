use crate::common::union_find::UnionFind;

pub fn solve(
    n: usize,
    p_list: &[(isize, isize)],
    q_list: &[(isize, isize)],
    ab_list: &[(usize, usize)],
) -> Vec<bool> {
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i..n {
            if does_intersect(p_list[i], q_list[i], p_list[j], q_list[j]) {
                uf.union(i, j)
            }
        }
    }
    let mut ans = vec![];
    for (a, b) in ab_list {
        if uf.find(*a) == uf.find(*b) {
            ans.push(true)
        } else {
            ans.push(false)
        }
    }
    ans
}

fn outer_product(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> isize {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}

fn does_intersect(
    p1: (isize, isize),
    p2: (isize, isize),
    p3: (isize, isize),
    p4: (isize, isize),
) -> bool {
    outer_product(p1, p2, p3) * outer_product(p1, p2, p4) <= 0
        && outer_product(p3, p4, p1) * outer_product(p3, p4, p2) <= 0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, &[(0, 4), (0, 1), (1, 2), (1, 0)], &[(4, 1), (2, 3), (3, 4), (2, 1)], &[(0, 1), (0, 3), (1, 2), (1, 3)], vec![true, false, true, false])]
    #[case(2, &[(0, 0), (1, 1)], &[(1, 1), (2, 2)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (0, 0)], &[(1, 1), (1, 1)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (1, 1)], &[(1, 1), (2, 0)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (0, 1)], &[(0, 1), (0, 2)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (0, 0)], &[(0, 1), (0, 1)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (0, 1)], &[(0, 1), (1, 1)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (1, 0)], &[(1, 0), (2, 0)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (0, 0)], &[(1, 0), (1, 0)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 0), (1, 0)], &[(1, 0), (1, 1)], &[(0, 1)], vec![true])]
    #[case(2, &[(0, 1), (1, 0)], &[(0, 2), (2, 0)], &[(0, 1)], vec![false])]
    fn it_works(
        #[case] n: usize,
        #[case] p_list: &[(isize, isize)],
        #[case] q_list: &[(isize, isize)],
        #[case] ab_list: &[(usize, usize)],
        #[case] expected: Vec<bool>,
    ) {
        assert_eq!(expected, solve(n, p_list, q_list, ab_list));
    }
}
