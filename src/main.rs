use antbook::chapter3::section7::numbers::solve;
use proconio::input;

fn main() {
    input! {
        n: usize
    }
    println!("{}", solve(n));
}
