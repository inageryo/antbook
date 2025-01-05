use antbook::chapter3::section1::lower_bound::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_list: [usize; n],
        k: usize
    }
    let ans = solve(&a_list, k);
    println!("{ans}");
}
