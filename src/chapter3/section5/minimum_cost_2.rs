use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: isize = 10isize.pow(12);
#[derive(Clone)]
struct Edge {
    to: usize,
    capacity: isize,
    cost: isize,
    rev: usize,
}
pub fn solve(
    n: usize,
    s: usize,
    t: usize,
    f: usize,
    network: &[(usize, usize, usize, usize)],
) -> Option<usize> {
    let mut graph = vec![vec![]; n];
    for (f, t, c, d) in network {
        let rev1 = graph[*t].len();
        let rev2 = graph[*f].len();
        graph[*f].push(Edge {
            to: *t,
            capacity: *c as isize,
            cost: *d as isize,
            rev: rev1,
        });
        graph[*t].push(Edge {
            to: *f,
            capacity: 0,
            cost: -(*d as isize),
            rev: rev2,
        });
    }
    let mut dist = vec![0; n];
    let mut previous_e = vec![0; n];
    let mut previous_v = vec![0; n];
    let mut h = vec![0; n];
    let mut ans = 0usize;
    let mut f = f as isize;
    while f > 0 {
        let mut pq = BinaryHeap::new();
        dist.fill(INF);
        dist[s] = 0;
        pq.push(Reverse((0, s)));
        while let Some(Reverse((d, v))) = pq.pop() {
            if dist[v] < d {
                continue;
            }
            for (i, e) in graph[v].iter().enumerate() {
                if e.capacity > 0 && dist[e.to] > dist[v] + e.cost + h[v] - h[e.to] {
                    dist[e.to] = dist[v] + e.cost + h[v] - h[e.to];
                    previous_v[e.to] = v;
                    previous_e[e.to] = i;
                    pq.push(Reverse((dist[e.to], e.to)));
                }
            }
        }
        if dist[t] == INF {
            return None;
        }
        for v in 0..n {
            h[v] += dist[v];
        }
        let mut d = f;
        let mut v = t;
        while v != s {
            d = d.min(graph[previous_v[v]][previous_e[v]].capacity);
            v = previous_v[v];
        }
        f -= d;
        ans += (d * h[t]) as usize;
        v = t;
        while v != s {
            let rev = graph[previous_v[v]][previous_e[v]].rev;
            graph[previous_v[v]][previous_e[v]].capacity -= d;
            graph[v][rev].capacity += d;
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
