use antbook::chapter3::section6::coneology::solve;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        circle_list: [(f64, f64, f64); n]
    }
    let ans = solve(n, &circle_list);
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|e| { *e + 1 }).format(" "));
}
