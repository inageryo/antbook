use crate::common::min_cost_flow::MinCostFlow;

pub fn solve(n: usize, m: usize, z_list: &[Vec<usize>]) -> f64 {
    let v = n + m * n + 2;
    let mut min_cost_flow = MinCostFlow::new(v);
    for (i, r) in z_list.iter().enumerate() {
        min_cost_flow.add_edge(0, i + 1, 1, 0);
        for (j, c) in r.iter().enumerate() {
            for k in 0..n {
                min_cost_flow.add_edge(i + 1, n + 1 + n * j + k, 1, (c * (k + 1)) as isize);
                if i == 0 {
                    min_cost_flow.add_edge(n + 1 + n * j + k, v - 1, 1, 0);
                }
            }
        }
    }
    min_cost_flow.min_cost_flow(0, v - 1, n as isize).unwrap() as f64 / n as f64
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 4, &[vec![100, 100, 100, 1], vec![99, 99, 99, 1], vec![98, 98, 98, 1]], 2.)]
    #[case(3, 4, &[vec![1, 100, 100, 100], vec![99, 1, 99, 99], vec![98, 98, 1, 98]], 1.)]
    #[case(3, 4, &[vec![1, 100, 100, 100], vec![1, 99, 99, 99], vec![98, 1, 98, 98]], 1.3333333333333333)]
    #[case(1, 1, &[vec![10]], 10.)]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] z_list: &[Vec<usize>],
        #[case] expected: f64,
    ) {
        assert_eq!(expected, solve(n, m, z_list));
    }
}
