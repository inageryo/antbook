use antbook::chapter3::section2::physics_experiment::solve;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        r: usize,
        t: usize,
    }
    println!(
        "{}",
        solve(n, h, r, t)
            .iter()
            .map(|x| { format!("{:.2}", x) })
            .join(" ")
    );
}
