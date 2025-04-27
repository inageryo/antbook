const G: f64 = 9.8;
const EPS: f64 = 1e-10;

pub fn solve(v: f64, x: f64, y: f64, obstacle_list: &[(f64, f64, f64, f64)]) -> bool {
    let mut ans = is_ok(x, y, v, x, y, obstacle_list);
    for (l, _, r, t) in obstacle_list.iter() {
        ans |= is_ok(*l, *t, v, x, y, obstacle_list);
        ans |= is_ok(*r, *t, v, x, y, obstacle_list);
    }
    ans
}

fn is_ok(qx: f64, qy: f64, v: f64, x: f64, y: f64, obstacle_list: &[(f64, f64, f64, f64)]) -> bool {
    let a = G * G / 4.;
    let b = G * qy - v * v;
    let c = qx * qx + qy * qy;
    let mut d = b * b - 4. * a * c;
    if d < 0. && d > -EPS {
        d = 0.
    };
    if d < 0. {
        return false;
    }
    let l = [1., -1.];
    for e in l {
        let tp = (-b + e * d.sqrt()) / (2. * a);
        if tp <= 0. {
            continue;
        }
        let t = tp.sqrt();
        let vx = qx / t;
        let vy = (qy + G * t * t / 2.) / t;
        let yt = get_y(vy, x / vx);
        if yt < y - EPS {
            continue;
        }
        let mut ret = true;
        for (l, b, r, t) in obstacle_list.iter() {
            if *l >= x {
                continue;
            }
            if *r >= x && *t >= y && *b <= yt {
                ret = false;
            }
            let yl = get_relative_position(*b, *t, get_y(vy, *l / vx));
            let yr = get_relative_position(*b, *t, get_y(vy, *r / vx));
            let xh = get_relative_position(*l, *r, vx * (vy / G));
            let yh = get_relative_position(*b, *t, get_y(vy, vy / G));
            if (xh == 0 && yh >= 0 && yl < 0) || (yl * yr <= 0) {
                ret = false;
            }
        }
        if ret {
            return true;
        }
    }
    false
}

fn get_y(vy: f64, t: f64) -> f64 {
    vy * t - G * t * t / 2.
}

fn get_relative_position(lb: f64, ub: f64, a: f64) -> isize {
    if a < lb + EPS {
        -1
    } else if a > ub - EPS {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(7., 3., 1., &[], true)]
    #[case(7., 3., 1., &[(1., 1., 2., 2.)], false)]
    #[case(7., 3., 1., &[(0., 0., 2., 2.)], false)]
    #[case(7., 3., 1., &[(1., 0., 2., 1.)], true)]
    #[case(7., 3., 1., &[(1., 2., 2., 3.)], true)]
    #[case(10., 1., 0., &[(1., 1., 2., 2.)], true)]
    fn it_works(
        #[case] v: f64,
        #[case] x: f64,
        #[case] y: f64,
        #[case] obstacle_list: &[(f64, f64, f64, f64)],
        #[case] expected: bool,
    ) {
        assert_eq!(expected, solve(v, x, y, obstacle_list));
    }
}
