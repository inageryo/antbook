use crate::common::min_cost_flow::MinCostFlow;

pub fn solve(n: usize, roads: &[(usize, usize, usize)]) -> usize {
    let mut min_cost_flow = MinCostFlow::new(n);
    for (from, to, cost) in roads {
        min_cost_flow.add_edge(*from, *to, 1, *cost as isize);
        min_cost_flow.add_edge(*to, *from, 1, *cost as isize);
    }
    min_cost_flow
        .min_cost_flow(0, n - 1, 2)
        .unwrap()
        .unsigned_abs()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(4, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (0, 2, 2), (1, 3, 2)], 6)]
    #[case(4, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (0, 2, 2), (1, 3, 2), (0, 3, 1)], 4)]
    #[case(4, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (0, 2, 4), (0, 3, 5)], 8)]
    fn it_works(
        #[case] n: usize,
        #[case] roads: &[(usize, usize, usize)],
        #[case] expected: usize,
    ) {
        assert_eq!(solve(n, roads), expected);
    }
}
