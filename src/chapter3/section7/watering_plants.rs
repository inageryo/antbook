pub fn solve(circles: &[(f64, f64, f64)]) -> f64 {
    let mut lb = 0.;
    let mut ub = 10000.;
    for _ in 0..100 {
        let mid = (lb + ub) / 2.;
        if can_cover(mid, circles) {
            ub = mid;
        } else {
            lb = mid;
        }
    }
    ub
}

fn get_covered_circles_set(x: f64, y: f64, r: f64, circles: &[(f64, f64, f64)]) -> usize {
    let mut s = 0usize;
    for (i, (xi, yi, ri)) in circles.iter().enumerate() {
        if *ri <= r {
            let dx = x - xi;
            let dy = y - yi;
            let dr = r - ri;
            if dx * dx + dy * dy <= dr * dr {
                s |= 1usize << i;
            }
        }
    }
    s
}

fn can_cover(r: f64, circles: &[(f64, f64, f64)]) -> bool {
    let mut set_list = vec![0usize];
    for (i, (xi, yi, ri)) in circles.iter().enumerate() {
        for (j, (xj, yj, rj)) in circles.iter().enumerate().take(i) {
            if ri < &r && rj < &r {
                let x1 = xi;
                let x2 = xj;
                let y1 = yi;
                let y2 = yj;
                let r1 = r - ri;
                let r2 = r - rj;
                let dx = x2 - x1;
                let dy = y2 - y1;
                let a = dx * dx + dy * dy;
                let b = ((r1 * r1 - r2 * r2) / a + 1.) / 2.;
                let d = r1 * r1 / a - b * b;
                if d >= 0. {
                    let d_sqrt = d.sqrt();
                    let x3 = x1 + dx * b;
                    let y3 = y1 + dy * b;
                    let x4 = -dy * d_sqrt;
                    let y4 = dx * d_sqrt;
                    let ij = (1 << i) | (1 << j);
                    set_list.push(get_covered_circles_set(x3 - x4, y3 - y4, r, circles) | ij);
                    set_list.push(get_covered_circles_set(x3 + x4, y3 + y4, r, circles) | ij);
                }
            }
        }
    }
    for (i, (xi, yi, ri)) in circles.iter().enumerate() {
        if ri <= &r {
            set_list.push(get_covered_circles_set(*xi, *yi, r, circles) | 1 << i);
        }
    }
    for (i, set_i) in set_list.iter().enumerate() {
        for set_j in set_list.iter().take(i) {
            if (set_i | set_j) == (1 << circles.len()) - 1 {
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
    #[case(&[(20., 10., 2.), (20., 20., 2.), (40., 10., 3.)], 7.0)]
    #[case(&[(20., 10., 3.), (30., 10., 3.), (40., 10., 3.)], 8.0)]
    #[case(&[(20., 10., 2.), (20., 20., 2.)], 2.0)]
    fn it_works(#[case] items: &[(f64, f64, f64)], #[case] expected: f64) {
        assert_eq!(expected, solve(items));
    }
}
