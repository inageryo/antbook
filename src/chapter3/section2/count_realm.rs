use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const MOVE: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn solve(
    w: usize,
    h: usize,
    n: usize,
    x1: &mut [usize],
    x2: &mut [usize],
    y1: &mut [usize],
    y2: &mut [usize],
) -> usize {
    let cw = compress(n, w as isize, x1, x2);
    let ch = compress(n, h as isize, y1, y2);
    let mut field = vec![vec![false; ch]; cw];
    for i in 0..n {
        for ex in field.iter_mut().take(x2[i] + 1).skip(x1[i]) {
            for ey in ex.iter_mut().take(y2[i] + 1).skip(y1[i]) {
                *ey = true;
            }
        }
    }
    let mut ans = 0usize;
    for i in 0..cw {
        for j in 0..ch {
            if !field[i][j] {
                ans += 1;
                field[i][j] = true;
                let mut q = VecDeque::new();
                q.push_back((i, j));
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in MOVE.iter() {
                        let xi = x as isize;
                        let yi = y as isize;
                        if 0 <= xi + dx
                            && xi + dx < cw as isize
                            && 0 <= yi + dy
                            && yi + dy < ch as isize
                        {
                            let nx = (xi + dx) as usize;
                            let ny = (yi + dy) as usize;
                            if !field[nx][ny] {
                                field[nx][ny] = true;
                                q.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
        }
    }
    ans
}

fn compress(n: usize, w: isize, x1: &mut [usize], x2: &mut [usize]) -> usize {
    let mut x_set = HashSet::new();
    for i in 0..n {
        for d in -1..=1 {
            let nx1 = x1[i] as isize + d;
            if 1 <= nx1 && nx1 <= w {
                x_set.insert(nx1 as usize);
            }
            let nx2 = x2[i] as isize + d;
            if 1 <= nx2 && nx2 <= w {
                x_set.insert(nx2 as usize);
            }
        }
    }
    let x_list = x_set.iter().sorted().copied().collect::<Vec<_>>();
    for x in x1.iter_mut() {
        let idx = x_list.binary_search(x).unwrap();
        *x = idx;
    }
    for x in x2.iter_mut() {
        let idx = x_list.binary_search(x).unwrap();
        *x = idx;
    }
    x_list.len()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    struct IntParameter {
        w: usize,
        h: usize,
        n: usize,
    }

    #[rstest]
    #[case(IntParameter {w: 10, h: 10, n: 5}, &mut [1, 1, 4, 9, 10], &mut [6, 10, 4, 9, 10], &mut [4, 8, 1, 1, 6], &mut [4, 8, 10, 5, 10], 6)]
    #[case(IntParameter {w: 1, h: 1, n: 1}, &mut [1], &mut [1], &mut [1], &mut [1], 0)]
    #[case(IntParameter {w: 10, h: 20, n: 2}, &mut [5, 1], &mut [5, 10], &mut [1, 10], &mut [20, 10], 4)]
    #[case(IntParameter {w: 1_000_000, h: 1_000_000, n: 1}, &mut [1], &mut [1], &mut [1], &mut [100], 1)]
    #[case(IntParameter {w: 1_000_000, h: 1_000_000, n: 5}, &mut [1, 1, 400_000, 999_999, 1_000_000], &mut [600_000, 1_000_000, 400_000, 999_999, 1_000_000], &mut [400_000, 800_000, 1, 1, 600_000], &mut [400_000, 800_000, 1_000_000, 599_999, 1_000_000], 6)]
    fn it_works(
        #[case] int_parameter: IntParameter,
        #[case] x1: &mut [usize],
        #[case] x2: &mut [usize],
        #[case] y1: &mut [usize],
        #[case] y2: &mut [usize],
        #[case] expected: usize,
    ) {
        assert_eq!(
            expected,
            solve(
                int_parameter.w,
                int_parameter.h,
                int_parameter.n,
                x1,
                x2,
                y1,
                y2
            )
        );
    }
}
