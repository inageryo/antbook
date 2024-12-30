use antbook::chapter2::section2::fence_repair::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l_lsit: [usize; n],
    }
    let ans = solve(&l_lsit);
    println!("{ans}");
}
