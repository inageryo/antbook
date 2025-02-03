use antbook::chapter3::section5::asteroids::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
        positions: [(Usize1, Usize1); k]
    }
    println!("{}", solve(n, &positions));
}
