use antbook::chapter2::section5::roadblocks::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..r {
        input! {
            s: Usize1,
            t: Usize1,
            c: usize,
        }
        graph[s].push((t, c));
        graph[t].push((s, c));
    }
    let ans = solve(n, &graph);
    println!("{ans}");
}
