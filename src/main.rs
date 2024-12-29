use antbook::chapter1::lottery::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k_list: [usize; n],
    }
    let ans = solve(m, k_list);
    println!("{ans}");
}
