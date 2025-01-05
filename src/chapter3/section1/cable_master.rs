pub fn solve(k: usize, l_list: &[f64]) -> f64 {
    let l_list = l_list
        .iter()
        .map(|&e| (e * 100.) as usize)
        .collect::<Vec<usize>>();
    let mut lb = 0usize;
    let mut ub = *l_list.iter().max().unwrap() + 1;
    while (ub - lb) > 1 {
        let mid = (ub + lb) / 2;
        if is_ok(mid, k, &l_list) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    lb as f64 / 100.
}

fn is_ok(v: usize, k: usize, l_list: &[usize]) -> bool {
    let mut count = 0usize;
    for l in l_list.iter() {
        count += l / v;
        if count >= k {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(11, &[8.02, 7.43, 4.57, 5.39], 2.)]
    #[case(1, &[8.02, 7.43, 4.57, 5.39], 8.02)]
    #[case(100, &[8.02, 7.43, 4.57, 5.39], 0.25)]
    fn it_works(#[case] k: usize, #[case] l_list: &[f64], #[case] expected: f64) {
        assert_eq!(expected, solve(k, l_list));
    }
}
