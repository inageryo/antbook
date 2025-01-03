use antbook::chapter2::section6::is_prime::solve;
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
