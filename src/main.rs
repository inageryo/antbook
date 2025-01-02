use antbook::chapter2::section5::bipartite_graph::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..n {
        input! {
            s: usize,
            t: usize,
        }
        graph[s].push(t);
        graph[t].push(s);
    }
    if solve(n, &graph) {
        println!("Yes");
    } else {
        println!("No");
    };
}
