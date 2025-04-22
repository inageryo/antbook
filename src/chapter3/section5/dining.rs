use crate::common::max_flow::MaxFlow;

pub fn solve(
    n: usize,
    f: usize,
    d: usize,
    food_preference: &[(usize, usize)],
    drink_preference: &[(usize, usize)],
) -> usize {
    let v_count = f + 2 * n + d + 2;
    let mut max_flow = MaxFlow::new(v_count);
    for i in 1..=f {
        max_flow.add_edge(0, i, 1);
        max_flow.add_edge(i, 0, 0);
    }
    for (i, j) in food_preference {
        max_flow.add_edge(1 + *j, f + 1 + *i, 1);
        max_flow.add_edge(f + 1 + *i, 1 + *j, 0);
    }
    for i in 1..=n {
        max_flow.add_edge(f + i, f + n + i, 1);
        max_flow.add_edge(f + n + i, f + i, 0);
    }
    for (i, j) in drink_preference {
        max_flow.add_edge(f + n + 1 + *i, f + 2 * n + 1 + *j, 1);
        max_flow.add_edge(f + 2 * n + 1 + *j, f + n + 1 + *i, 0);
    }
    for i in 1..=d {
        max_flow.add_edge(f + 2 * n + i, f + 2 * n + d + 1, 1);
        max_flow.add_edge(f + 2 * n + d + 1, f + 2 * n + i, 0);
    }
    max_flow.max_flow(0, f + 2 * n + d + 1)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(4, 3, 3, &[(0, 0), (0, 1), (1, 1), (1, 2), (2, 0), (2, 2), (3, 0), (3, 2)], &[(0, 0), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1), (3, 2)], 3)]
    #[case(1, 1, 1, &[(0, 0)], &[(0, 0)], 1)]
    #[case(4, 4, 4, &[(0, 0), (0, 3), (1, 1), (1, 2), (2, 0), (2, 2), (3, 0), (3, 2)], &[(0, 0), (0, 3), (1, 0), (1, 1), (2, 0), (2, 1), (3, 2)], 4)]
    fn it_works(
        #[case] n: usize,
        #[case] f: usize,
        #[case] d: usize,
        #[case] food_preference: &[(usize, usize)],
        #[case] drink_preference: &[(usize, usize)],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, f, d, food_preference, drink_preference));
    }
}
