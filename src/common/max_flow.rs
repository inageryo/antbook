const INF: usize = 10usize.pow(12);
pub struct MaxFlow {
    graph: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

#[derive(Clone)]
struct Edge {
    to: usize,
    capacity: isize,
    rev: usize,
}

impl MaxFlow {
    pub fn new(n: usize) -> MaxFlow {
        MaxFlow {
            graph: vec![vec![]; n],
            used: vec![false; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: isize) {
        let fr = self.graph[from].len();
        let tr = self.graph[to].len();
        self.graph[from].push(Edge {
            to,
            capacity,
            rev: tr,
        });
        self.graph[to].push(Edge {
            to: from,
            capacity: 0,
            rev: fr,
        });
    }

    fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        if v == t {
            return f;
        }
        self.used[v] = true;
        for i in 0..self.graph[v].len() {
            let e = self.graph[v][i].clone();
            if !self.used[e.to] && e.capacity > 0 {
                let d = self.dfs(e.to, t, f.min(e.capacity as usize));
                if d > 0 {
                    self.graph[v][i].capacity -= d as isize;
                    self.graph[e.to][e.rev].capacity += d as isize;
                    return d;
                }
            }
        }
        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        loop {
            self.used.fill(false);
            let f = self.dfs(s, t, INF);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut max_flow = MaxFlow::new(7);
        max_flow.add_edge(0, 1, 10);
        max_flow.add_edge(0, 2, 2);
        max_flow.add_edge(1, 2, 6);
        max_flow.add_edge(1, 3, 6);
        max_flow.add_edge(2, 4, 5);
        max_flow.add_edge(3, 2, 3);
        max_flow.add_edge(3, 4, 8);
        assert_eq!(11, max_flow.max_flow(0, 4));
        max_flow = MaxFlow::new(7);
        max_flow.add_edge(0, 1, 10);
        max_flow.add_edge(0, 2, 2);
        max_flow.add_edge(1, 2, 6);
        max_flow.add_edge(1, 3, 6);
        max_flow.add_edge(2, 4, 5);
        max_flow.add_edge(3, 2, 3);
        max_flow.add_edge(3, 4, 8);
        assert_eq!(10, max_flow.max_flow(0, 1));
        max_flow = MaxFlow::new(7);
        max_flow.add_edge(0, 1, 10);
        max_flow.add_edge(0, 2, 2);
        max_flow.add_edge(1, 2, 6);
        max_flow.add_edge(1, 3, 7);
        max_flow.add_edge(2, 4, 5);
        max_flow.add_edge(3, 2, 3);
        max_flow.add_edge(3, 4, 8);
        assert_eq!(12, max_flow.max_flow(0, 4));
    }
}
