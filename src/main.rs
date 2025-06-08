use antbook::chapter3::section7::stock_charts::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        price_list: [[usize; k]; n]
    }
    println!("{}", solve(n, &price_list));
}
