const INF: isize = 10isize.pow(12);

pub fn solve(
    n: usize,
    m: usize,
    bills: &[(isize, isize, usize)],
    shelters: &[(isize, isize, usize)],
    plan: &[Vec<usize>],
) -> Option<Vec<Vec<usize>>> {
    let v = n + m + 1;
    let mut g = vec![vec![INF; v]; v];
    let mut prev = vec![vec![0; v]; v];
    let mut used = vec![false; v];
    for j in 0..m {
        let mut sum = 0;
        for i in 0..n {
            let c = ((bills[i].0 - shelters[j].0).unsigned_abs()
                + (bills[i].1 - shelters[j].1).unsigned_abs()
                + 1) as isize;
            g[i][n + j] = c;
            if plan[i][j] > 0 {
                g[n + j][i] = -c;
            }
            sum += c;
        }
        if sum > 0 {
            g[n + m][n + j] = 0;
        }
        if sum < shelters[j].2 as isize {
            g[n + j][n + m] = 0;
        }
    }
    for (i, r) in prev.iter_mut().enumerate() {
        for c in r.iter_mut() {
            *c = i;
        }
    }
    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if g[i][j] > g[i][k] + g[k][j] {
                    g[i][j] = g[i][k] + g[k][j];
                    prev[i][j] = prev[k][j];
                    if i == j && g[i][j] < 0 {
                        let mut optimized_plan = plan.to_owned();
                        used.fill(false);
                        let mut vi = i;
                        while !used[vi] {
                            used[vi] = true;
                            if vi != n + m && prev[i][vi] != n + m {
                                if vi >= n {
                                    optimized_plan[prev[i][vi]][vi - n] += 1;
                                } else {
                                    optimized_plan[vi][prev[i][vi] - n] -= 1;
                                }
                            }
                            vi = prev[i][vi]
                        }
                        return Some(optimized_plan);
                    }
                }
            }
        }
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
    #[case(3, 4, &[(-3, 3, 5), (-2, -2, 6), (2, 2, 5)], &[(-1, 1, 6), (1, 1, 4), (-2, -2 ,7), (0, -1, 3)], &[vec![3, 1, 1, 0], vec![0, 0, 6, 0], vec![0, 3, 0, 2]], Some(vec![vec![3, 0, 1, 1], vec![0, 0, 6, 0], vec![0, 4, 0, 1]]))]
    #[case(3, 4, &[(-3, 3, 5), (-2, -2, 6), (2, 2, 5)], &[(-1, 1, 6), (1, 1, 5), (-2, -2 ,7), (0, -1, 3)], &[vec![3, 1, 1, 0], vec![0, 0, 6, 0], vec![0, 3, 0, 2]], Some(vec![vec![3, 0, 1, 1], vec![0, 0, 6, 0], vec![0, 4, 0, 1]]))]
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
