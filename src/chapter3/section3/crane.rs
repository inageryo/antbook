pub fn solve(
    n: usize,
    c: usize,
    l_list: &[usize],
    s_list: &[usize],
    a_list: &[usize],
) -> Vec<(f64, f64)> {
    let mut size = 1;
    while size < n {
        size *= 2;
    }
    let mut tree = vec![(0., 0., 0.); 2 * size - 1];
    for (i, e) in l_list.iter().enumerate() {
        update(i, (0., *e as f64, 0.), size, &mut tree);
    }
    let mut ans = vec![];
    for i in 0..c {
        let k = s_list[i];
        let a = (a_list[i] as f64 - 180.).to_radians();
        update(k, (0., l_list[i] as f64, a), size, &mut tree);
        ans.push((tree[0].0, tree[0].1));
    }
    ans
}

fn update(k: usize, v: (f64, f64, f64), size: usize, tree: &mut [(f64, f64, f64)]) {
    let mut k = k + size - 1;
    tree[k] = v;
    while k > 0 {
        k = (k - 1) / 2;
        let v1 = tree[k * 2 + 1];
        let v2 = tree[k * 2 + 2];
        let nx = v1.0 + v1.2.cos() * v2.0 - v1.2.sin() * v2.1;
        let ny = v1.1 + v1.2.sin() * v2.0 + v1.2.cos() * v2.1;
        let na = v1.2 + v2.2;
        tree[k] = (nx, ny, na);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(2, 1, &[10, 5], &[0], &[90], &[(5.00, 10.00)])]
    #[case(3, 2, &[5, 5, 5], &[0, 1], &[270, 90], &[(-10.00, 5.00), (-5.00, 10.00)])]
    #[case(4, 3, &[5, 5, 5, 5], &[0, 1, 2], &[270, 90, 270], &[(-15.00, 5.00), (-5.00, 15.00), (-10.00, 10.00)])]
    #[case(4, 3, &[5, 5, 5, 5], &[0, 1, 1], &[270, 90, 270], &[(-15.00, 5.00), (-5.00, 15.00), (-5.00, -5.00)])]
    fn it_works(
        #[case] n: usize,
        #[case] c: usize,
        #[case] l_list: &[usize],
        #[case] s_list: &[usize],
        #[case] a_list: &[usize],
        #[case] expected: &[(f64, f64)],
    ) {
        let actual = solve(n, c, l_list, s_list, a_list);
        for i in 0..c {
            assert!((actual[i].0 - expected[i].0).abs() < 1e-10);
            assert!((actual[i].1 - expected[i].1).abs() < 1e-10);
        }
    }
}
