use crate::common::min_cost_flow::MinCostFlow;

pub fn solve(
    n: usize,
    s: usize,
    t: usize,
    f: usize,
    network: &[(usize, usize, usize, usize)],
) -> Option<isize> {
    let mut min_cost_flow = MinCostFlow::new(n);
    for (f, t, c, d) in network {
        min_cost_flow.add_edge(*f, *t, *c as isize, *d as isize);
    }
    min_cost_flow.min_cost_flow(s, t, f as isize)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(5, 0, 4, 9, &[(0, 1, 10, 2), (0, 2, 2, 4), (1, 2, 6, 6), (1, 3, 6, 2), (2, 4, 5, 2), (3, 2, 3, 3), (3, 4, 8, 6)], Some(80))]
    #[case(5, 0, 1, 9, &[(0, 1, 10, 2), (0, 2, 2, 4), (1, 2, 6, 6), (1, 3, 6, 2), (2, 4, 5, 2), (3, 2, 3, 3), (3, 4, 8, 6)], Some(18))]
    #[case(5, 1, 3, 9, &[(0, 1, 10, 2), (0, 2, 2, 4), (1, 2, 6, 6), (1, 3, 6, 2), (2, 4, 5, 2), (3, 2, 3, 3), (3, 4, 8, 6)], None)]
    fn it_works(
        #[case] n: usize,
        #[case] s: usize,
        #[case] t: usize,
        #[case] f: usize,
        #[case] network: &[(usize, usize, usize, usize)],
        #[case] expected: Option<isize>,
    ) {
        assert_eq!(expected, solve(n, s, t, f, network));
    }
}
