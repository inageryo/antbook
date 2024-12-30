use antbook::chapter2::section2::sarumans_army::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        x_lsit: [usize; n],
    }
    let ans = solve(r, &x_lsit);
    println!("{ans}");
}
