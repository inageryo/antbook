use antbook::chapter2::section3::lis::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_list: [usize; n],
    }
    let ans = solve(n, &a_list);
    println!("{ans}");
}
