use antbook::chapter3::section2::huge_knapsack::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w_list: [usize; n],
        v_list: [usize; n],
        w_limit: usize
    }
    println!("{}", solve(n, &w_list, &v_list, w_limit));
}
