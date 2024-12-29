use antbook::chapter2::partial_sum::solve;
use proconio::input;

fn main() {
    input! {
        n: isize,
        a_list: [isize; n],
        k: isize,
    }
    let ans = solve(&a_list, k);
    println!("{ans}");
}
