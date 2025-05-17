use crate::common::bipartile_graph::BipartileGraph;

const D: [(isize, isize); 4] = [(-1, -1), (-1, 0), (1, -1), (1, 0)];

pub fn solve(m: usize, n: usize, seats: &[Vec<char>]) -> usize {
    let v = m * n;
    let mut count = 0;
    let mut graph = BipartileGraph::new(v);
    for (i, row) in seats.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if *seat == '.' {
                count += 1;
                for &(x, y) in D.iter() {
                    let ni = i as isize + y;
                    let nj = j as isize + x;
                    if 0 <= ni
                        && ni < m as isize
                        && 0 <= nj
                        && nj < n as isize
                        && seats[ni as usize][nj as usize] == '.'
                    {
                        graph.add_edge(i * n + j, ni as usize * n + nj as usize);
                    }
                }
            }
        }
    }
    count - graph.bipartile_matching()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(2, 3, &[vec!['.', '.', '.'], vec!['.', '.', '.']], 4)]
    #[case(2, 3, &[vec!['X', '.', 'X'], vec!['X', 'X', 'X']], 1)]
    #[case(2, 3, &[vec!['X', '.', 'X'], vec!['X', '.', 'X']], 2)]
    #[case(2, 3, &[vec!['X', '.', '.'], vec!['X', 'X', 'X']], 1)]
    #[case(2, 3, &[vec!['X', '.', 'X'], vec!['.', 'X', 'X']], 1)]
    #[case(3, 3, &[vec!['X', '.', '.'], vec!['.', '.', '.'], vec!['X', '.', '.']], 4)]
    #[case(3, 3, &[vec!['.', '.', '.'], vec!['.', '.', '.'], vec!['.', '.', '.']], 6)]
    #[case(3, 3, &[vec!['X', 'X', 'X'], vec!['X', 'X', 'X'], vec!['X', 'X', 'X']], 0)]
    fn it_works(
        #[case] m: usize,
        #[case] n: usize,
        #[case] seats: &[Vec<char>],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(m, n, seats));
    }
}
