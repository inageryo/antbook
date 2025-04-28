use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Debug)]
struct Event {
    x: f64,
    i: usize,
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.partial_cmp(&other.x).unwrap()
    }
}

impl PartialEq<Self> for Event {
    fn eq(&self, other: &Self) -> bool {
        other.x == self.x
    }
}

impl Eq for Event {}

pub fn solve(n: usize, circle_list: &[(f64, f64, f64)]) -> Vec<usize> {
    let mut events = vec![];
    for (i, e) in circle_list.iter().enumerate() {
        events.push(Event { x: e.0 - e.2, i });
        events.push(Event {
            x: e.0 + e.2,
            i: i + n,
        });
    }
    events.sort();
    let mut outers: BTreeSet<Event> = BTreeSet::new();
    let mut ans = vec![];
    for event in events {
        let index = event.i % n;
        if event.i < n {
            let mut lower_range = outers.range((Unbounded, Excluded(&event)));
            let mut upper_range = outers.range((Excluded(&event), Unbounded));
            if let Some(e) = lower_range.next() {
                if is_inside(circle_list[index], circle_list[e.i]) {
                    continue;
                }
            }
            if let Some(e) = upper_range.next() {
                if is_inside(circle_list[index], circle_list[e.i]) {
                    continue;
                }
            }
            outers.insert(event);
            ans.push(index);
        } else {
            outers.remove(&event);
        }
    }
    ans.sort();
    ans
}

fn is_inside(i: (f64, f64, f64), j: (f64, f64, f64)) -> bool {
    // the center of circle j is outside of circle i
    let dx = i.0 - j.0;
    let dy = i.1 - j.1;
    dx * dx + dy * dy <= j.2 * j.2
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, &[(0., -2., 1.), (0., 3., 3.), (0., 0., 10.), (0., 1.5, 1.), (50., 50., 10.)], vec![2, 4])]
    #[case(2, &[(0., 0., 1.), (10., 10., 1.)], vec![0, 1])]
    #[case(2, &[(0., 0., 1.), (10., 10., 1.)], vec![0, 1])]
    #[case(3, &[(0., 0., 3.), (0., 0., 2.), (0., 0., 1.)], vec![0])]
    #[case(4, &[(0., 0., 10.), (0., 5., 1.), (0., -5., 1.), (10., 10., 1.)], vec![0, 3])]
    fn it_works(
        #[case] n: usize,
        #[case] circle_list: &[(f64, f64, f64)],
        #[case] expected: Vec<usize>,
    ) {
        assert_eq!(expected, solve(n, circle_list));
    }
}
