use antbook::chapter2::section6::sugoroku::solve;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }
    let ans = solve(a, b);
    println!("{ans}");
}
