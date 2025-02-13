pub fn solve(
    n: usize,
    f: usize,
    d: usize,
    food_preference: &[(usize, usize)],
    drink_preference: &[(usize, usize)],
) -> usize {
    // fixme matrix -> list
    let v_count = f + 2 * n + d + 2;
    let mut graph = vec![vec![-1; v_count]; v_count];
    for i in 1..=f {
        graph[0][i] = 1;
        graph[i][0] = 0;
    }
    for (i, j) in food_preference {
        graph[1 + *j][f + 1 + *i] = 1;
        graph[f + 1 + *i][1 + *j] = 0;
    }
    for i in 1..=n {
        graph[f + i][f + n + i] = 1;
        graph[f + n + i][f + i] = 0;
    }
    for (i, j) in drink_preference {
        graph[f + n + 1 + *i][f + 2 * n + 1 + *j] = 1;
        graph[f + 2 * n + 1 + *j][f + n + 1 + *i] = 0;
    }
    for i in 1..=d {
        graph[f + 2 * n + i][f + 2 * n + d + 1] = 1;
        graph[f + 2 * n + d + 1][f + 2 * n + i] = 0;
    }
    let mut ans = 0;
    let mut seen = vec![false; v_count];
    loop {
        seen.fill(false);
        let f = dfs(
            0,
            f + 2 * n + d + 1,
            10usize.pow(12),
            v_count,
            &mut seen,
            &mut graph,
        );
        if f == 0 {
            return ans;
        }
        ans += f;
    }
}

fn dfs(
    s: usize,
    t: usize,
    f: usize,
    n: usize,
    seen: &mut [bool],
    graph: &mut [Vec<isize>],
) -> usize {
    if s == t {
        return f;
    }
    seen[s] = true;
    for i in 0..n {
        let e = graph[s][i];
        if !seen[i] && e > 0 {
            let d = dfs(i, t, f.min(e as usize), n, seen, graph);
            if d > 0 {
                graph[s][i] -= d as isize;
                graph[i][s] += d as isize;
                return d;
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
    #[case(4, 3, 3, &[(0, 0), (0, 1), (1, 1), (1, 2), (2, 0), (2, 2), (3, 0), (3, 2)], &[(0, 0), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1), (3, 2)], 3)]
    #[case(1, 1, 1, &[(0, 0)], &[(0, 0)], 1)]
    #[case(4, 4, 4, &[(0, 0), (0, 3), (1, 1), (1, 2), (2, 0), (2, 2), (3, 0), (3, 2)], &[(0, 0), (0, 3), (1, 0), (1, 1), (2, 0), (2, 1), (3, 2)], 4)]
    fn it_works(
        #[case] n: usize,
        #[case] f: usize,
        #[case] d: usize,
        #[case] food_preference: &[(usize, usize)],
        #[case] drink_preference: &[(usize, usize)],
        #[case] expected: usize,
    ) {
        assert_eq!(
            expected,
            solve(n, f, d, &food_preference, &drink_preference)
        );
    }
}
