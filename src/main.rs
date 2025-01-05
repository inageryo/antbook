use antbook::chapter3::section1::cable_master::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        l_list: [f64; n],
    }
    let ans = solve(k, &l_list);
    println!("{:.2}", ans);
}
