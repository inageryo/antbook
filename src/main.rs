use antbook::chapter2::section4::fence_repair::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l_list: [usize; n],
    }
    let ans = solve(&l_list);
    println!("{ans}");
}
