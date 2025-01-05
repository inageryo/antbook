use antbook::chapter3::section1::aggressive_cows::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x_list: [usize; n],
    }
    println!("{}", solve(n, m, &mut x_list));
}
