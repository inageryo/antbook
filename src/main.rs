use antbook::chapter2::section2::best_cow_line::solve;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let ans = solve(s);
    println!("{ans}");
}
