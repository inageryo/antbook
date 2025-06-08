use crate::common::bipartile_graph::BipartileGraph;

pub fn solve(n: usize, price_list: &[Vec<usize>]) -> usize {
    let v = n * 2;
    let mut graph = BipartileGraph::new(v);
    for (i, p1_list) in price_list.iter().enumerate() {
        for (j, p2_list) in price_list.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut all_above = true;
            for (k, p1) in p1_list.iter().enumerate() {
                if p1 >= &p2_list[k] {
                    all_above = false;
                    break;
                }
            }
            if all_above {
                graph.add_edge(i, n + j);
            }
        }
    }
    n - graph.bipartile_matching()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(5, &[vec![1, 1], vec![2, 2], vec![4, 4], vec![5, 4], vec![4, 1]], 2)]
    #[case(5, &[vec![1, 1], vec![2, 2], vec![5, 4], vec![4, 4], vec![4, 1]], 2)]
    #[case(3, &[vec![1, 1], vec![2, 2], vec![3, 3]], 1)]
    #[case(3, &[vec![1, 1], vec![3, 3], vec![2, 2,]], 1)]
    #[case(3, &[vec![1, 8], vec![4, 3], vec![5, 0]], 3)]
    fn it_works(#[case] n: usize, #[case] price_list: &[Vec<usize>], #[case] expected: usize) {
        assert_eq!(expected, solve(n, price_list));
    }
}
