pub fn solve(n: usize, s: usize, t: usize, network: &[(usize, usize, usize)]) -> usize {
    let mut graph = vec![vec![-1; n]; n];
    for (f, t, c) in network {
        graph[*f][*t] = *c as isize;
        graph[*t][*f] = 0;
    }
    let mut ans = 0;
    let mut seen = vec![false; n];
    loop {
        seen.fill(false);
        let f = dfs(s, t, 10usize.pow(12), n, &mut seen, &mut graph);
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
    #[case(5, 0, 4, &[(0, 1, 10), (0, 2, 2), (1, 2, 6), (1, 3, 6), (2, 4, 5), (3, 2, 3), (3, 4, 8)], 11)]
    #[case(5, 0, 1, &[(0, 1, 10), (0, 2, 2), (1, 2, 6), (1, 3, 6), (2, 4, 5), (3, 2, 3), (3, 4, 8)], 10)]
    #[case(5, 0, 4, &[(0, 1, 10), (0, 2, 2), (1, 2, 6), (1, 3, 7), (2, 4, 5), (3, 2, 3), (3, 4, 8)], 12)]
    fn it_works(
        #[case] n: usize,
        #[case] s: usize,
        #[case] t: usize,
        #[case] network: &[(usize, usize, usize)],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, s, t, network));
    }
}
