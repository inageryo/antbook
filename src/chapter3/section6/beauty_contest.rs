use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

pub fn solve(n: usize, points: &[(isize, isize)]) -> usize {
    let mut points = points
        .iter()
        .map(|&(x1, y1)| Point { x: x1, y: y1 })
        .collect::<Vec<_>>();
    points.sort();
    let mut convex_hull = vec![Point { x: 0, y: 0 }; n * 2];
    let mut k = 0usize;
    for point in points.iter() {
        while k > 1 && get_outer_product(&convex_hull[k - 1], &convex_hull[k - 2], point) <= 0 {
            k -= 1;
        }
        convex_hull[k] = point.clone();
        k += 1;
    }
    let t = k;
    for point in points.iter().rev().skip(1) {
        while k > t && get_outer_product(&convex_hull[k - 1], &convex_hull[k - 2], point) <= 0 {
            k -= 1;
        }
        convex_hull[k] = point.clone();
        k += 1;
    }
    convex_hull.resize(k - 1, Point { x: 0, y: 0 });
    let mut ans = 0;
    for (i, p1) in convex_hull.iter().enumerate() {
        for p2 in convex_hull.iter().take(i) {
            ans = ans.max(get_power_of_distance(p1, p2));
        }
    }
    ans
}

fn get_outer_product(p1: &Point, p2: &Point, p3: &Point) -> isize {
    let a = p2.x - p1.x;
    let b = p2.y - p1.y;
    let c = p3.x - p1.x;
    let d = p3.y - p1.y;
    a * d - b * c
}

fn get_power_of_distance(p1: &Point, p2: &Point) -> usize {
    let a = p1.x - p2.x;
    let b = p1.y - p2.y;
    (a * a + b * b) as usize
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(8, &[(0, 5), (1, 8), (3, 4), (5, 0), (6, 2), (6, 6), (8, 3), (8, 7)], 80)]
    #[case(8, &[(8, 7), (0, 5), (1, 8), (6, 6), (3, 4), (5, 0), (6, 2), (8, 3)], 80)]
    #[case(2, &[(0, 0), (1, 1)], 2)]
    #[case(3, &[(0, 0), (1, 1), (2, 2)], 8)]
    fn it_works(#[case] n: usize, #[case] points: &[(isize, isize)], #[case] expected: usize) {
        assert_eq!(expected, solve(n, points));
    }
}
