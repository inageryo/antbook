use antbook::chapter2::section3::partial_function::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        md: usize
    }
    let ans = solve(n, m, md);
    println!("{ans}");
}
