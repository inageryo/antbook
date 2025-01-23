use antbook::chapter3::section5::job_allocation::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        can: [[bool; k]; n]
    }
    println!("{}", solve(n, k, &can));
}
