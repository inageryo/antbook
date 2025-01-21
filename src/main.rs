use antbook::chapter3::section4::minimizing_maximizer::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        list: [(Usize1, Usize1); m]
    }
    println!("{}", solve(n, &list));
}
