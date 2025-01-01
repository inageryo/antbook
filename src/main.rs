use antbook::chapter2::section4::food_chain::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        info_list: [(usize, isize, isize); k],
    }
    let ans = solve(n, &info_list);
    println!("{ans}");
}
