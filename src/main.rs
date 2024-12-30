use antbook::chapter2::section2::scheduling::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }
    let ans = solve(n, &s, &t);
    println!("{ans}");
}
