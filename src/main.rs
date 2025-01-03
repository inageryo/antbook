use antbook::chapter2::section6::count_prime::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let ans = solve(n);
    println!("{ans}");
}
