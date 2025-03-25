use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: isize = 10isize.pow(12);

pub struct MinCostFlow {
    n: usize,
    graph: Vec<Vec<Edge>>,
    dist: Vec<isize>,
    previous_e: Vec<usize>,
    previous_v: Vec<usize>,
    h: Vec<isize>,
}
#[derive(Clone)]
struct Edge {
    to: usize,
    capacity: isize,
    cost: isize,
    rev: usize,
}

impl MinCostFlow {
    // fixme: refactor
    pub fn new(n: usize) -> Self {
        MinCostFlow {
            n,
            graph: vec![vec![]; n],
            dist: vec![0; n],
            previous_e: vec![0; n],
            previous_v: vec![0; n],
            h: vec![0; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: isize, cost: isize) {
        let rev1 = self.graph[to].len();
        let rev2 = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            capacity,
            cost,
            rev: rev1,
        });
        self.graph[to].push(Edge {
            to: from,
            capacity: 0,
            cost: -cost,
            rev: rev2,
        });
    }

    pub fn min_cost_flow(&mut self, s: usize, t: usize, f: isize) -> Option<isize> {
        let mut ans = 0;
        let mut f = f;
        while f > 0 {
            let mut pq = BinaryHeap::new();
            self.dist.fill(INF);
            self.dist[s] = 0;
            pq.push(Reverse((0, s)));
            while let Some(Reverse((d, v))) = pq.pop() {
                if self.dist[v] < d {
                    continue;
                }
                for (i, e) in self.graph[v].iter().enumerate() {
                    if e.capacity > 0
                        && self.dist[e.to] > self.dist[v] + e.cost + self.h[v] - self.h[e.to]
                    {
                        self.dist[e.to] = self.dist[v] + e.cost + self.h[v] - self.h[e.to];
                        self.previous_v[e.to] = v;
                        self.previous_e[e.to] = i;
                        pq.push(Reverse((self.dist[e.to], e.to)));
                    }
                }
            }
            if self.dist[t] == INF {
                return None;
            }
            for v in 0..self.n {
                self.h[v] += self.dist[v];
            }
            let mut d = f;
            let mut v = t;
            while v != s {
                d = d.min(self.graph[self.previous_v[v]][self.previous_e[v]].capacity);
                v = self.previous_v[v];
            }
            f -= d;
            ans += d * self.h[t];
            v = t;
            while v != s {
                let rev = self.graph[self.previous_v[v]][self.previous_e[v]].rev;
                self.graph[self.previous_v[v]][self.previous_e[v]].capacity -= d;
                self.graph[v][rev].capacity += d;
                v = self.previous_v[v];
            }
        }
        Some(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut min_cost_flow = MinCostFlow::new(5);
        min_cost_flow.add_edge(0, 1, 10, 2);
        min_cost_flow.add_edge(0, 2, 2, 4);
        min_cost_flow.add_edge(1, 2, 6, 6);
        min_cost_flow.add_edge(1, 3, 6, 2);
        min_cost_flow.add_edge(2, 4, 5, 2);
        min_cost_flow.add_edge(3, 2, 3, 3);
        min_cost_flow.add_edge(3, 4, 8, 6);
        assert_eq!(80, min_cost_flow.min_cost_flow(0, 4, 9).unwrap());
        min_cost_flow = MinCostFlow::new(5);
        min_cost_flow.add_edge(0, 1, 10, 2);
        min_cost_flow.add_edge(0, 2, 2, 4);
        min_cost_flow.add_edge(1, 2, 6, 6);
        min_cost_flow.add_edge(1, 3, 6, 2);
        min_cost_flow.add_edge(2, 4, 5, 2);
        min_cost_flow.add_edge(3, 2, 3, 3);
        min_cost_flow.add_edge(3, 4, 8, 6);
        assert_eq!(18, min_cost_flow.min_cost_flow(0, 1, 9).unwrap());
        min_cost_flow = MinCostFlow::new(5);
        min_cost_flow.add_edge(0, 1, 10, 2);
        min_cost_flow.add_edge(0, 2, 2, 4);
        min_cost_flow.add_edge(1, 2, 6, 6);
        min_cost_flow.add_edge(1, 3, 6, 2);
        min_cost_flow.add_edge(2, 4, 5, 2);
        min_cost_flow.add_edge(3, 2, 3, 3);
        min_cost_flow.add_edge(3, 4, 8, 6);
        assert_eq!(None, min_cost_flow.min_cost_flow(1, 3, 9));
    }
}
