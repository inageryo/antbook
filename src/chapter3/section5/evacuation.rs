use crate::common::bipartile_graph::BipartileGraph;
use std::collections::VecDeque;

const D: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn solve(x: usize, y: usize, field: &[Vec<char>]) -> Option<usize> {
    let n = x * y;
    let mut doors = vec![];
    let mut people = vec![];
    let mut dist = vec![vec![vec![vec![-1; y]; x]; y]; x];
    for xx in 0..x {
        for yy in 0..y {
            if field[xx][yy] == 'D' {
                doors.push((xx, yy));
                bfs(xx, yy, &mut dist[xx][yy], field, x, y);
            } else if field[xx][yy] == '.' {
                people.push((xx, yy));
            }
        }
    }
    let md = doors.len();
    let mp = people.len();
    let mut bg = BipartileGraph::new(n * md + mp);
    for (i, ed) in doors.iter().enumerate() {
        for (j, ep) in people.iter().enumerate() {
            if dist[ed.0][ed.1][ep.0][ep.1] >= 0 {
                for k in dist[ed.0][ed.1][ep.0][ep.1] as usize..=n {
                    bg.add_edge((k - 1) * md + i, n * md + j);
                }
            }
        }
    }
    if mp == 0 {
        return Some(0);
    }
    let mut count = 0;
    bg.matching.fill(-1);
    for v in 0..n * md {
        bg.used.fill(false);
        if bg.dfs(v) {
            count += 1;
            if count == mp {
                return Some(v / md + 1);
            }
        }
    }
    None
}

fn bfs(x: usize, y: usize, d: &mut [Vec<isize>], field: &[Vec<char>], mx: usize, my: usize) {
    let mut q = VecDeque::new();
    d[x][y] = 0;
    q.push_back((x, y));
    while let Some((x, y)) = q.pop_front() {
        for e in D.iter() {
            let nx = x as isize + e.0;
            let ny = y as isize + e.1;
            if 0 <= nx
                && nx < mx as isize
                && 0 <= ny
                && ny < my as isize
                && field[nx as usize][ny as usize] == '.'
                && d[nx as usize][ny as usize] < 0
            {
                d[nx as usize][ny as usize] = d[x][y] + 1;
                q.push_back((nx as usize, ny as usize));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(5, 5, &[vec!['X', 'X', 'D', 'X', 'X'], vec!['X', '.', '.', '.', 'X'], vec!['D', '.', '.', '.', 'X'], vec!['X', '.', '.', '.', 'D'], vec!['X', 'X', 'D', 'X', 'X']], Some(3))]
    #[case(5, 12, &[vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'], vec!['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'D'], vec!['X', '.', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'], vec!['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'], vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X']], Some(21))]
    #[case(5, 5, &[vec!['X', 'D', 'X', 'X', 'X'], vec!['X', '.', 'X', '.', 'D'], vec!['X', 'X', '.', 'X', 'X'], vec!['D', '.', 'X', '.', 'X'], vec!['X', 'X', 'X', 'D', 'X']], None)]
    fn it_works(
        #[case] x: usize,
        #[case] y: usize,
        #[case] field: &[Vec<char>],
        #[case] expected: Option<usize>,
    ) {
        assert_eq!(expected, solve(x, y, field));
    }
}
