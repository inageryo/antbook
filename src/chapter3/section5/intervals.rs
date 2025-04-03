use crate::common::min_cost_flow::MinCostFlow;
use std::collections::HashSet;

const INF: isize = 10isize.pow(12);

pub fn solve(n: usize, k: usize, intervals: &[(usize, usize, usize)]) -> usize {
    let mut x_set: HashSet<usize> = HashSet::new();
    for (a, b, _) in intervals.iter() {
        x_set.insert(*a);
        x_set.insert(*b);
    }
    let mut x_list = Vec::from_iter(x_set);
    x_list.sort();
    let m = x_list.len();
    let mut min_cost_flow = MinCostFlow::new(m + 2);
    min_cost_flow.add_edge(0, 1, k as isize, 0);
    min_cost_flow.add_edge(m, m + 1, k as isize, 0);
    for i in 1..m {
        min_cost_flow.add_edge(i, i + 1, INF, 0);
    }
    let mut ans = 0;
    for (a, b, w) in intervals.iter() {
        let u = x_list.binary_search(a).unwrap() + 1;
        let v = x_list.binary_search(b).unwrap() + 1;
        min_cost_flow.add_edge(v, u, 1, *w as isize);
        min_cost_flow.add_edge(0, v, 1, 0);
        min_cost_flow.add_edge(u, m + 1, 1, 0);
        ans -= *w as isize;
    }
    ans += min_cost_flow
        .min_cost_flow(0, m + 1, (k + n) as isize)
        .unwrap();
    -ans as usize
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 1, &[(1, 2, 2), (2, 3, 4), (3, 4, 8)], 14)]
    #[case(3, 1, &[(1, 3, 2), (2, 3, 4), (3, 4, 8)], 12)]
    #[case(3, 2, &[(1, 100_000, 100_000), (1, 150, 301), (100, 200, 300)], 100_301)]
    #[case(5, 1, &[(1, 100, 101), (2, 99, 100), (3, 4, 999), (4, 5, 999), (5, 6, 999)], 2997)]
    #[case(5, 2, &[(1, 100, 101), (2, 99, 100), (3, 4, 999), (4, 5, 999), (5, 6, 999)], 3098)]
    fn it_works(
        #[case] n: usize,
        #[case] k: usize,
        #[case] intervals: &[(usize, usize, usize)],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, k, intervals));
    }
}
