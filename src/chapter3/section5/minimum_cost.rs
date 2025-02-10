const INF: isize = 10isize.pow(12);
pub fn solve(
    n: usize,
    s: usize,
    t: usize,
    f: usize,
    network: &[(usize, usize, usize, usize)],
) -> Option<usize> {
    // (to, capacity, cost, rev)
    let mut graph = vec![vec![]; n];
    for (f, t, c, d) in network {
        let rev1 = graph[*t].len();
        let rev2 = graph[*f].len();
        graph[*f].push((*t, *c as isize, *d as isize, rev1));
        graph[*t].push((*f, 0, -(*d as isize), rev2));
    }
    let mut dist = vec![0; n];
    let mut previous_e = vec![0; n];
    let mut previous_v = vec![0; n];
    let mut ans = 0usize;
    let mut f = f as isize;
    while f > 0 {
        dist.fill(INF);
        dist[s] = 0;
        let mut updated = true;
        while updated {
            updated = false;
            for i in 0..n {
                if dist[i] == INF {
                    continue;
                }
                for (j, e) in graph[i].iter().enumerate() {
                    if e.1 > 0 && dist[e.0] > dist[i] + e.2 {
                        dist[e.0] = dist[i] + e.2;
                        previous_v[e.0] = i;
                        previous_e[e.0] = j;
                        updated = true;
                    }
                }
            }
        }
        if dist[t] == INF {
            return None;
        }
        let mut d = f;
        let mut v = t;
        while v != s {
            d = d.min(graph[previous_v[v]][previous_e[v]].1);
            v = previous_v[v];
        }
        f -= d;
        ans += (d * dist[t]) as usize;
        v = t;
        while v != s {
            let rev = graph[previous_v[v]][previous_e[v]].3;
            graph[previous_v[v]][previous_e[v]].1 -= d;
            graph[v][rev].1 += d;
            v = previous_v[v];
        }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(5, 0, 4, 9, &[(0, 1, 10, 2), (0, 2, 2, 4), (1, 2, 6, 6), (1, 3, 6, 2), (2, 4, 5, 2), (3, 2, 3, 3), (3, 4, 8, 6)], Some(80))]
    #[case(5, 0, 1, 9, &[(0, 1, 10, 2), (0, 2, 2, 4), (1, 2, 6, 6), (1, 3, 6, 2), (2, 4, 5, 2), (3, 2, 3, 3), (3, 4, 8, 6)], Some(18))]
    #[case(5, 1, 3, 9, &[(0, 1, 10, 2), (0, 2, 2, 4), (1, 2, 6, 6), (1, 3, 6, 2), (2, 4, 5, 2), (3, 2, 3, 3), (3, 4, 8, 6)], None)]
    fn it_works(
        #[case] n: usize,
        #[case] s: usize,
        #[case] t: usize,
        #[case] f: usize,
        #[case] network: &[(usize, usize, usize, usize)],
        #[case] expected: Option<usize>,
    ) {
        assert_eq!(expected, solve(n, s, t, f, network));
    }
}
