use antbook::chapter3::section4::traveling_salesman_problem::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [[usize; n]; n]
    }
    println!("{}", solve(n, &d));
}
