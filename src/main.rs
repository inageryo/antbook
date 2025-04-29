use antbook::chapter3::section6::beauty_contest::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(isize, isize); n]
    }
    println!("{}", solve(n, &points));
}
