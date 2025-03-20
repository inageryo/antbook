use crate::common::max_flow::MaxFlow;

pub fn solve(
    n: usize,
    cost_list: &[(usize, usize)],
    exchange_cost_list: &[(usize, usize, usize)],
) -> usize {
    let v_count = n + 2;
    let mut max_flow = MaxFlow::new(v_count);
    for (i, (a_cost, b_cost)) in cost_list.iter().enumerate() {
        max_flow.add_edge(i + 1, n + 1, *a_cost as isize);
        max_flow.add_edge(0, i + 1, *b_cost as isize);
    }
    for (a, b, w) in exchange_cost_list.iter() {
        max_flow.add_edge(*a, *b, *w as isize);
        max_flow.add_edge(*b, *a, *w as isize);
    }
    max_flow.max_flow(0, n + 1)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(3, &[(1, 10), (2, 10), (10, 3)], &[(2, 3, 1000)], 13)]
    #[case(3, &[(1, 10), (2, 10), (10, 3)], &[(2, 3, 1)], 7)]
    #[case(3, &[(1, 10), (2, 10), (10, 3)], &[(2, 3, 7)], 13)]
    #[case(3, &[(1, 10), (2, 10), (10, 3)], &[(2, 3, 8)], 13)]
    #[case(3, &[(10, 1), (10, 2), (3, 10)], &[(2, 3, 1)], 7)]
    fn it_works(
        #[case] n: usize,
        #[case] cost_list: &[(usize, usize)],
        #[case] exchange_cost_list: &[(usize, usize, usize)],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, &cost_list, &exchange_cost_list));
    }
}
