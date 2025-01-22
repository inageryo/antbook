use antbook::chapter3::section5::max_traffic::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        e: usize,
        network: [(usize, usize, usize); e]
    }
    println!("{}", solve(n, s, t, &network));
}
