use antbook::chapter2::section7::crazy_rows::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut matrix: [[u8; n]; n],
    }
    let ans = solve(n, &mut matrix);
    println!("{ans}");
}
