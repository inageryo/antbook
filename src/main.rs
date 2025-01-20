use antbook::chapter3::section4::count_path::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        graph: [[usize; n]; n]
    }
    println!("{}", solve(n, k, &graph));
}
