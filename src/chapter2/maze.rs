use std::collections::VecDeque;

const MOVE: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
pub fn solve(h: usize, w: usize, maze: &[Vec<char>]) -> usize {
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for (i, r) in maze.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 'S' {
                sx = i;
                sy = j;
            }
            if *c == 'G' {
                gx = i;
                gy = j;
            }
        }
    }
    let mut seen = vec![vec![10usize.pow(10); w]; h];
    seen[sx][sy] = 0;
    let mut q = VecDeque::new();
    q.push_back((sx, sy));
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if x == gx && y == gy {
            return seen[gx][gy];
        }
        for (dx, dy) in MOVE.iter() {
            let xi = x as isize;
            let yi = y as isize;
            if 0 <= xi + dx && xi + dx < h as isize && 0 <= yi + dy && yi + dy < w as isize {
                let nx = (xi + dx) as usize;
                let ny = (yi + dy) as usize;
                if maze[nx][ny] != '#' && seen[nx][ny] > seen[x][y] + 1 {
                    seen[nx][ny] = seen[x][y] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 2, vec![vec!['S', 'G']], 1)]
    #[case(3, 3, vec![vec!['.', 'S', '.'], vec!['.', '.', '.'], vec!['.', '.', 'G']], 3)]
    #[case(3, 3, vec![vec!['S', '#', 'G'], vec!['.', '#', '.'], vec!['.', '.', '.']], 6)]
    #[case(3, 3, vec![vec!['S', '.', 'G'], vec!['.', '#', '.'], vec!['.', '.', '.']], 2)]
    fn it_works(
        #[case] h: usize,
        #[case] w: usize,
        #[case] maze: Vec<Vec<char>>,
        #[case] expected: usize,
    ) {
        let result = solve(h, w, &maze);
        assert_eq!(expected, result);
    }
}
