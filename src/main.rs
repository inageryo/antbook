use antbook::chapter2::section4::expedition::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        p: usize,
        a_list: [usize; n],
        b_list: [usize; n],
    }
    let ans = solve(n, l, p, &a_list, &b_list);
    println!("{ans}");
}
