pub fn solve(n: usize, k: usize, can: &[Vec<bool>]) -> usize {
    let size = n + k + 2;
    let mut graph = vec![vec![false; size]; size];
    for i in 1..=n {
        graph[0][i] = true;
    }
    for gi in graph.iter_mut().take(n + k + 1).skip(n + 1) {
        gi[size - 1] = true;
    }
    for i in 0..n {
        for j in 0..k {
            if can[i][j] {
                graph[i + 1][j + n + 1] = true;
            }
        }
    }
    let mut ans = 0;
    let mut seen = vec![false; size];
    loop {
        seen.fill(false);
        let f = dfs(0, size - 1, size, &mut seen, &mut graph);
        if f == 0 {
            return ans;
        }
        ans += f;
    }
}

fn dfs(s: usize, t: usize, n: usize, seen: &mut [bool], graph: &mut [Vec<bool>]) -> usize {
    if s == t {
        return 1;
    }
    seen[s] = true;
    for i in 0..n {
        let e = graph[s][i];
        if !seen[i] && e {
            let d = dfs(i, t, n, seen, graph);
            if d > 0 {
                graph[s][i] = false;
                graph[i][s] = true;
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
    #[case(3, 3, &[vec![true, true, false], vec![true, false, true], vec![false, true, false]], 3)]
    #[case(1, 1, &[vec![true]], 1)]
    #[case(1, 2, &[vec![true, false]], 1)]
    #[case(1, 2, &[vec![false, false]], 0)]
    #[case(3, 3, &[vec![true, true, false], vec![false, false, true], vec![false, false, true]], 2)]
    fn it_works(
        #[case] n: usize,
        #[case] k: usize,
        #[case] can: &[Vec<bool>],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, k, can));
    }
}
