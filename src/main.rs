use antbook::chapter3::section5::the_windys::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        z_list: [[usize; m]; n]
    }
    println!("{:.6}", solve(n, m, &z_list))
}
