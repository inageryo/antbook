const INF: f64 = 1e12;

pub fn solve(p1_list: &[(f64, f64)], p2_list: &[(f64, f64)]) -> f64 {
    let mut x_list = vec![];
    for p1 in p1_list.iter() {
        x_list.push(p1.0);
    }
    for p2 in p2_list.iter() {
        x_list.push(p2.0);
    }
    x_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ans = 0.;
    let min1 = p1_list
        .iter()
        .map(|x| x.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min2 = p2_list
        .iter()
        .map(|x| x.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max1 = p1_list
        .iter()
        .map(|x| x.0)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max2 = p2_list
        .iter()
        .map(|x| x.0)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    for (i, a) in x_list.iter().enumerate().take(x_list.len() - 1) {
        let b = x_list[i + 1];
        let c = (a + b) / 2.;
        if min1 <= c && c <= max1 && min2 <= c && c <= max2 {
            let fa = get_width(p1_list, *a) * get_width(p2_list, *a);
            let fb = get_width(p1_list, b) * get_width(p2_list, b);
            let fc = get_width(p1_list, c) * get_width(p2_list, c);
            ans += (b - a) * (fa + 4. * fc + fb) / 6.
        }
    }
    ans
}

fn get_width(p_list: &[(f64, f64)], x: f64) -> f64 {
    let mut lb = INF;
    let mut ub = -INF;
    let len = p_list.len();
    for (i, (x1, y1)) in p_list.iter().enumerate() {
        let (x2, y2) = &p_list[(i + 1) % len];
        if (x1 - x) * (x2 - x) <= 0. && x1 != x2 {
            let y = y1 + (y2 - y1) * (x - x1) / (x2 - x1);
            lb = lb.min(y);
            ub = ub.max(y);
        }
    }
    (ub - lb).max(0.)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[(7., 2.), (3., 3.), (0., 2.), (3., 1.)], &[(4., 2.), (0., 1.), (8., 1.)], 4.708333333333333)]
    #[case(&[(0., 1.), (2., 2.), (3., 1.)], &[(4., 0.), (5., 1.), (6., 0.)], 0.)]
    #[case(&[(0., 1.), (2., 2.), (3., 1.)], &[(0., 0.), (5., 1.), (6., 0.)], 0.5)]
    fn it_works(
        #[case] p1_list: &[(f64, f64)],
        #[case] p2_list: &[(f64, f64)],
        #[case] expected: f64,
    ) {
        assert_eq!(expected, solve(p1_list, p2_list));
    }
}
