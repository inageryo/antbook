use antbook::chapter3::section4::fibonacci::solve;
use proconio::input;

fn main() {
    input! {
        n: usize
    }
    println!("{}", solve(n));
}
