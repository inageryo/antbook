use antbook::chapter2::section7::millionaire::solve;
use proconio::input;

fn main() {
    input! {
        m: usize,
        p: f64,
        x: usize,
    }
    let ans = solve(m, p, x);
    println!("{ans}");
}
