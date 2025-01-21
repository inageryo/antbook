use antbook::chapter3::section4::matrix_power_series::solve;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [[usize; n]; n]
    }
    for v in solve(n, k, m, &a) {
        println!("{}", v.iter().join(" "));
    }
}
