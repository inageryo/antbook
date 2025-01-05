pub fn solve(k: usize, items: &mut [(usize, usize)]) -> f64 {
    let mut lb = 0f64;
    let mut ub = 1e7;
    let mut ans = 0.;
    for _ in 0..100 {
        let mid = (ub + lb) / 2.;
        if is_ok(mid, k, items, &mut ans) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    ans
}

fn is_ok(m: f64, k: usize, items: &mut [(usize, usize)], ans: &mut f64) -> bool {
    items.sort_by(|a, b| {
        get_comparison_key(b.0, b.1, m)
            .partial_cmp(&get_comparison_key(a.0, a.1, m))
            .unwrap()
    });
    let mut w_sum = 0.;
    let mut v_sum = 0.;
    for (w, v) in items.iter().take(k) {
        w_sum += *w as f64;
        v_sum += *v as f64;
    }
    if v_sum / w_sum > m {
        *ans = v_sum / w_sum;
        true
    } else {
        false
    }
}

fn get_comparison_key(w: usize, v: usize, m: f64) -> f64 {
    v as f64 - m * w as f64
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(2, &mut [(2, 2), (5, 3), (2, 1)], 0.75)]
    #[case(1, &mut [(2, 2), (5, 3), (2, 1)], 1.)]
    #[case(3, &mut [(2, 2), (5, 3), (2, 1)], 0.6666666666666666)]
    fn it_works(#[case] k: usize, #[case] items: &mut [(usize, usize)], #[case] expected: f64) {
        assert_eq!(expected, solve(k, items));
    }
}
