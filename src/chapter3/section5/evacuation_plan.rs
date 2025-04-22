use crate::common::min_cost_flow::MinCostFlow;
const INF: isize = 10isize.pow(12);

pub fn solve(
    n: usize,
    m: usize,
    bills: &[(isize, isize, usize)],
    shelters: &[(isize, isize, usize)],
    plan: &[Vec<usize>],
) -> Option<Vec<Vec<usize>>> {
    let mut planed_cost = 0usize;
    for (i, p) in plan.iter().enumerate() {
        for (j, e) in p.iter().enumerate() {
            planed_cost += *e
                * ((bills[i].0 - shelters[j].0).unsigned_abs()
                    + (bills[i].1 - shelters[j].1).unsigned_abs()
                    + 1);
        }
    }
    let mut f = 0;
    let mut min_cost_flow = MinCostFlow::new(n + m + 2);
    for (i, (x, y, b)) in bills.iter().enumerate() {
        f += b;
        min_cost_flow.add_edge(0, i + 1, *b as isize, 0);
        for (j, (p, q, c)) in shelters.iter().enumerate() {
            min_cost_flow.add_edge(
                i + 1,
                j + n + 1,
                INF,
                ((x - p).unsigned_abs() + (y - q).unsigned_abs() + 1) as isize,
            );
            if i == 0 {
                min_cost_flow.add_edge(j + n + 1, n + m + 1, *c as isize, 0);
            }
        }
    }
    let min_cost = min_cost_flow
        .min_cost_flow(0, n + m + 1, f as isize)
        .unwrap()
        .unsigned_abs();
    if planed_cost > min_cost {
        let mut optimized_plan = vec![vec![0; m]; n];
        for (i, r) in optimized_plan.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                *c = (INF - min_cost_flow.graph[i + 1][j + 1].capacity) as usize;
            }
        }
        return Some(optimized_plan);
    }
    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(3, 4, &[(-3, 3, 5), (-2, -2, 6), (2, 2, 5)], &[(-1, 1, 3), (1, 1, 4), (-2, -2 ,7), (0, -1, 3)], &[vec![3, 1, 1, 0], vec![0, 0, 6, 0], vec![0, 3, 0, 2]], Some(vec![vec![3, 0, 1, 1], vec![0, 0, 6, 0], vec![0, 4, 0, 1]]))]
    #[case(3, 4, &[(-3, 3, 5), (-2, -2, 6), (2, 2, 5)], &[(-1, 1, 3), (1, 1, 4), (-2, -2 ,7), (0, -1, 3)], &[vec![3, 0, 1, 1], vec![0, 0, 6, 0], vec![0, 4, 0, 1]], None)]
    #[case(3, 4, &[(-3, 3, 5), (-2, -2, 6), (2, 2, 5)], &[(-1, 1, 6), (1, 1, 4), (-2, -2 ,7), (0, -1, 3)], &[vec![3, 1, 1, 0], vec![0, 0, 6, 0], vec![0, 3, 0, 2]], Some(vec![vec![5, 0, 0, 0], vec![0, 0, 6, 0], vec![1, 4, 0, 0]]))]
    #[case(3, 4, &[(-3, 3, 5), (-2, -2, 6), (2, 2, 5)], &[(-1, 1, 6), (1, 1, 5), (-2, -2 ,7), (0, -1, 3)], &[vec![3, 1, 1, 0], vec![0, 0, 6, 0], vec![0, 3, 0, 2]], Some(vec![vec![5, 0, 0, 0], vec![0, 0, 6, 0], vec![0, 5, 0, 0]]))]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] bills: &[(isize, isize, usize)],
        #[case] shelters: &[(isize, isize, usize)],
        #[case] plan: &[Vec<usize>],
        #[case] expected: Option<Vec<Vec<usize>>>,
    ) {
        assert_eq!(expected, solve(n, m, bills, shelters, plan));
    }
}
