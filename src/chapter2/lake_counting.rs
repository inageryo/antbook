const MOVE: [(isize, isize); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
pub fn solve(h: usize, w: usize, garden: &mut Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for x in 0..h {
        for y in 0..w {
            if garden[x][y] == 'W' {
                count += 1;
                dfs(x, y, h, w, garden);
            }
        }
    }
    count
}

fn dfs(x: usize, y: usize, h: usize, w: usize, garden: &mut Vec<Vec<char>>) {
    if garden[x][y] == '.' {
        return;
    }
    garden[x][y] = '.';
    for (dx, dy) in MOVE.iter() {
        let xi = x as isize;
        let yi = y as isize;
        if 0 <= xi + dx && xi + dx < h as isize && 0 <= yi + dy && yi + dy < w as isize {
            dfs((xi + dx) as usize, (yi + dy) as usize, h, w, garden);
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 2, vec![vec!['.', '.']], 0)]
    #[case(3, 3, vec![vec!['.', '.', '.'], vec!['.', 'W', '.'], vec!['.', '.', '.']], 1)]
    #[case(3, 3, vec![vec!['W', '.', '.'], vec!['.', 'W', '.'], vec!['.', '.', 'W']], 1)]
    #[case(3, 3, vec![vec!['W', '.', '.'], vec!['.', '.', '.'], vec!['.', '.', 'W']], 2)]
    fn it_works(
        #[case] h: usize,
        #[case] w: usize,
        #[case] mut garden: Vec<Vec<char>>,
        #[case] expected: usize,
    ) {
        let result = solve(h, w, &mut garden);
        assert_eq!(expected, result);
    }
}
