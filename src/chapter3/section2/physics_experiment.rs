const G: f64 = 10.0;
pub fn solve(n: usize, h: usize, r: usize, t: usize) -> Vec<f64> {
    let mut y_list = vec![];
    for i in 0..n {
        y_list.push(get_y(h as f64, (t - i) as f64));
    }
    y_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for (i, y) in y_list.iter_mut().enumerate() {
        *y += 2. * (r * i) as f64 / 100.;
    }
    y_list
}

fn get_y(h: f64, t: f64) -> f64 {
    if t < 0. {
        return h;
    }
    let time = (2. * h / G).sqrt();
    let k = (t / time) as usize;
    let d = if k % 2 == 0 {
        t - k as f64 * time
    } else {
        k as f64 * time + time - t
    };
    h - G * d * d / 2.
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 10, 10, 100, vec![4.949366116653453])]
    #[case(2, 10, 10, 100, vec![4.949366116653453, 10.199872455486881])]
    fn it_works(
        #[case] n: usize,
        #[case] h: usize,
        #[case] r: usize,
        #[case] t: usize,
        #[case] expected: Vec<f64>,
    ) {
        assert_eq!(expected, solve(n, h, r, t));
    }
}
