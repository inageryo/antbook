use antbook::chapter3::section3::bubble_sort::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_list: [usize; n],
    }
    println!("{}", solve(n, &a_list));
}
