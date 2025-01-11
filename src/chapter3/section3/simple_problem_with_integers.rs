pub fn solve(
    n: usize,
    a_list: &[usize],
    q: usize,
    t_list: &[char],
    l_list: &[usize],
    r_list: &[usize],
    x_list: &[usize],
) -> Vec<usize> {
    let mut size = 1;
    while size < n {
        size *= 2;
    }
    let mut tree = vec![(0usize, 0usize); 2 * size - 1];
    let mut ans = vec![];
    for (i, a) in a_list.iter().enumerate() {
        update(i, i + 1, *a, 0, 0, size, &mut tree);
    }
    for i in 0..q {
        if t_list[i] == 'C' {
            update(l_list[i], r_list[i] + 1, x_list[i], 0, 0, size, &mut tree);
        } else {
            ans.push(sum(l_list[i], r_list[i] + 1, 0, 0, size, &tree));
        }
    }
    ans
}

// add x to [a, b)
fn update(a: usize, b: usize, x: usize, k: usize, l: usize, r: usize, tree: &mut [(usize, usize)]) {
    if a <= l && r <= b {
        tree[k].1 += x;
    } else if l < b && a < r {
        tree[k].0 += (b.min(r) - a.max(l)) * x;
        update(a, b, x, 2 * k + 1, l, (l + r) / 2, tree);
        update(a, b, x, 2 * k + 2, (l + r) / 2, r, tree);
    }
}

// sum of [a, b)
fn sum(a: usize, b: usize, k: usize, l: usize, r: usize, tree: &[(usize, usize)]) -> usize {
    if b <= l || r <= a {
        0
    } else if a <= l && r <= b {
        tree[k].1 * (r - l) + tree[k].0
    } else {
        (b.min(r) - a.max(l)) * tree[k].1
            + sum(a, b, k * 2 + 1, l, (l + r) / 2, tree)
            + sum(a, b, k * 2 + 2, (l + r) / 2, r, tree)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, &[5, 3, 7, 9], 2, &['C', 'Q'], &[0, 0], &[2, 3], &[1, 0], vec![27])]
    #[case(4, &[5, 3, 7, 9], 1, &['C'], &[0], &[2], &[1], vec![])]
    #[case(1, &[5], 3, &['Q', 'C', 'Q'], &[0, 0, 0], &[0, 0, 0], &[0, 1, 0], vec![5, 6])]
    #[case(8, &[5, 3, 7, 9, 6, 4, 1, 2], 3, &['Q', 'C', 'Q'], &[0, 0, 0], &[7, 4, 7], &[0, 1, 0], vec![37, 42])]
    fn it_works(
        #[case] n: usize,
        #[case] a_list: &[usize],
        #[case] q: usize,
        #[case] t_list: &[char],
        #[case] l_list: &[usize],
        #[case] r_list: &[usize],
        #[case] x_list: &[usize],
        #[case] expected: Vec<usize>,
    ) {
        assert_eq!(
            expected,
            solve(n, a_list, q, t_list, l_list, r_list, x_list)
        );
    }
}
