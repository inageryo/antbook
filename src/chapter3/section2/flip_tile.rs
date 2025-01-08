const MOVE: [(isize, isize); 5] = [(0, 0), (1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn solve(m: usize, n: usize, tiles: &[Vec<usize>]) -> Option<Vec<Vec<usize>>> {
    let mut ans = vec![];
    let mut num = usize::MAX;
    for i in 0..(1 << n) {
        let mut flip = vec![vec![0; n]; m];
        for j in 0..n {
            flip[0][n - 1 - j] = i >> j & 1;
        }
        if let Some(count) = get_count(m, n, tiles, &mut flip) {
            if num > count {
                num = count;
                ans = flip;
            }
        }
    }
    if num == usize::MAX {
        None
    } else {
        Some(ans)
    }
}

fn get_count(m: usize, n: usize, tiles: &[Vec<usize>], flip: &mut [Vec<usize>]) -> Option<usize> {
    for i in 1..m {
        for j in 0..n {
            if get_tile(m, n, (i - 1) as isize, j as isize, tiles, flip) != 0 {
                flip[i][j] = 1;
            }
        }
    }
    for j in 0..n {
        if get_tile(m, n, (m - 1) as isize, j as isize, tiles, flip) != 0 {
            return None;
        }
    }
    Some(flip.iter().map(|e| e.iter().sum::<usize>()).sum::<usize>())
}

fn get_tile(
    m: usize,
    n: usize,
    cm: isize,
    cn: isize,
    tiles: &[Vec<usize>],
    flip: &[Vec<usize>],
) -> usize {
    let mut c = tiles[cm as usize][cn as usize];
    for (dm, dn) in MOVE.iter() {
        if 0 <= cm + dm && cm + dm < m as isize && 0 <= cn + dn && cn + dn < n as isize {
            let nm = (cm + dm) as usize;
            let nn = (cn + dn) as usize;
            c += flip[nm][nn];
        }
    }
    c % 2
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, 4, &[vec![1, 0, 0, 1], vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![1, 0, 0, 1]], Some(vec![vec![0, 0, 0, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 1], vec![0, 0, 0, 0]]))]
    #[case(1, 4, &[vec![0, 0, 0, 0]], Some(vec![vec![0, 0, 0, 0]]))]
    #[case(1, 4, &[vec![1, 1, 0, 0]], Some(vec![vec![1, 0, 0, 0]]))]
    #[case(1, 4, &[vec![0, 1, 0, 0]], Some(vec![vec![0, 0, 1, 1]]))]
    #[case(2, 3, &[vec![0, 1, 0], vec![0, 0, 0]], None)]
    fn it_works(
        #[case] m: usize,
        #[case] n: usize,
        #[case] tiles: &[Vec<usize>],
        #[case] expected: Option<Vec<Vec<usize>>>,
    ) {
        assert_eq!(expected, solve(m, n, tiles));
    }
}
