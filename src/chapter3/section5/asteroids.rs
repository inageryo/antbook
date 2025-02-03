use crate::common::bipartile_graph::BipartileGraph;

pub fn solve(n: usize, positions: &[(usize, usize)]) -> usize {
    let mut bg = BipartileGraph::new(2 * n);
    for &(x, y) in positions.iter() {
        bg.add_edge(x, y);
    }
    bg.bipartile_matching()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(3, &[(0, 0), (0, 2), (1, 1), (2, 1)], 2)]
    #[case(1, &[(0, 0)], 1)]
    #[case(3, &[(0, 0), (0, 2), (1, 1), (2, 2)], 3)]
    #[case(3, &[(0, 0), (0, 1), (0, 2)], 1)]
    fn it_works(#[case] n: usize, #[case] positions: &[(usize, usize)], #[case] expected: usize) {
        assert_eq!(expected, solve(n, positions));
    }
}
