use antbook::chapter2::section7::bribe_the_prisoners::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        p: usize,
        q: usize,
        a_list: [Usize1; q],
    }
    let ans = solve(p, &a_list);
    println!("{ans}");
}
