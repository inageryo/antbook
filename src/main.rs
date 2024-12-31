use antbook::chapter2::section3::knapsack_2::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [(usize, usize); n],
        w: usize,
    }
    let ans = solve(n, &l, w);
    println!("{ans}");
}
