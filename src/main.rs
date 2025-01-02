use antbook::chapter2::section5::conscription::solve;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
    }
    let mut graph_m = vec![HashMap::<usize, usize>::new(); n];
    let mut graph_f = vec![HashMap::<usize, usize>::new(); m];
    for _ in 0..r {
        input! {
            x: usize,
            y: usize,
            d: usize,
        }
        let v = *graph_m[x].get(&y).unwrap_or(&0).max(&d);
        graph_m[x].insert(y, v);
        let v = *graph_f[y].get(&x).unwrap_or(&0).max(&d);
        graph_f[y].insert(x, v);
    }
    let ans = solve(n, m, &graph_m, &graph_f);
    println!("{ans}");
}
