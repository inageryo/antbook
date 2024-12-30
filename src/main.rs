use antbook::chapter2::section2::coins::solve;
use proconio::input;

fn main() {
    input! {
        c1: usize,
        c5: usize,
        c10: usize,
        c50: usize,
        c100: usize,
        c500: usize,
        a: usize
    }
    let ans = solve(c1, c5, c10, c50, c100, c500, a);
    println!("{ans}");
}
