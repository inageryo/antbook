use antbook::chapter2::section7::minimum_scalar_product::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v1: [isize; n],
        mut v2: [isize; n],
    }
    let ans = solve(n, &mut v1, &mut v2);
    println!("{ans}");
}
