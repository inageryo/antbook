pub struct BipartileGraph {
    n: usize,
    pub(crate) graph: Vec<Vec<usize>>,
    pub(crate) matching: Vec<isize>,
    pub(crate) used: Vec<bool>,
}

impl BipartileGraph {
    pub fn new(n: usize) -> BipartileGraph {
        BipartileGraph {
            n,
            graph: vec![vec![]; n],
            matching: vec![-1; n],
            used: vec![false; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        self.graph[to].push(from);
    }

    pub fn dfs(&mut self, v: usize) -> bool {
        self.used[v] = true;
        for i in 0..self.graph[v].len() {
            let e = self.graph[v][i];
            let m = self.matching[e];
            if m < 0 || self.used[m as usize] && self.dfs(m as usize) {
                self.matching[v] = e as isize;
                self.matching[e] = v as isize;
                return true;
            }
        }
        false
    }

    pub fn bipartile_matching(&mut self) -> usize {
        let mut res = 0;
        self.matching.fill(-1);
        for i in 0..self.n {
            if self.matching[i] < 0 {
                self.used.fill(false);
                if self.dfs(i) {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = BipartileGraph::new(6);
        graph.add_edge(0, 3);
        graph.add_edge(0, 4);
        graph.add_edge(1, 3);
        graph.add_edge(1, 5);
        graph.add_edge(2, 4);
        assert_eq!(3, graph.bipartile_matching());
        graph = BipartileGraph::new(2);
        graph.add_edge(0, 1);
        assert_eq!(1, graph.bipartile_matching());
        graph = BipartileGraph::new(3);
        graph.add_edge(0, 1);
        assert_eq!(1, graph.bipartile_matching());
        graph = BipartileGraph::new(3);
        assert_eq!(0, graph.bipartile_matching());
        let mut graph = BipartileGraph::new(6);
        graph.add_edge(0, 3);
        graph.add_edge(0, 4);
        graph.add_edge(1, 5);
        graph.add_edge(2, 5);
        assert_eq!(2, graph.bipartile_matching());
    }
}
