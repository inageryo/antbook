use antbook::chapter3::section5::intervals::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        intervals: [(usize, usize, usize); n]
    }
    println!("{}", solve(n, k, &intervals))
}
