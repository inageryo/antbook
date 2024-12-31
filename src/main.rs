use antbook::chapter2::section3::lcs::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let ans = solve(n, m, &s, &t);
    println!("{ans}");
}
