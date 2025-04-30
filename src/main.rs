use antbook::chapter3::section6::beauty_contest::solve2;
use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(isize, isize); n]
    }
    println!("{}", solve2(n, &points));
}
