pub fn solve(n: usize, graph: &[Vec<usize>]) -> bool {
    let mut seen = vec![-1; n];
    let mut ans = true;
    for i in 0..n {
        if seen[i] == -1 {
            seen[i] = 0;
            ans = ans && dfs(i, graph, &mut seen);
            if !ans {
                return ans;
            }
        }
    }
    ans
}

fn dfs(current: usize, graph: &[Vec<usize>], seen: &mut Vec<isize>) -> bool {
    let mut ret = true;
    for e in graph[current].iter() {
        if seen[current] == seen[*e] {
            return false;
        } else if seen[*e] == -1 {
            seen[*e] = (seen[current] - 1).abs();
            ret = ret && dfs(*e, graph, seen);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, &[vec![1, 2], vec![0, 2], vec![0, 1]], false)]
    #[case(4, &[vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]], true)]
    #[case(4, &[vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]], false)]
    #[case(1, &[vec![]], true)]
    fn it_works(#[case] n: usize, #[case] graph: &[Vec<usize>], #[case] expected: bool) {
        assert_eq!(expected, solve(n, graph));
    }
}
