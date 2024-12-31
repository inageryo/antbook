use antbook::chapter2::section3::combination_with_replacement::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_list: [usize; n],
        md: usize
    }
    let ans = solve(n, m, &a_list, md);
    println!("{ans}");
}
