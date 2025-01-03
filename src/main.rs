use antbook::chapter2::section6::carmichael_numbers::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if solve(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
