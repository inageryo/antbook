use antbook::chapter3::section1::maximize_average::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut items: [(usize, usize); n],
    }
    println!("{}", solve(k, &mut items));
}
