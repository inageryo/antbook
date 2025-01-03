use antbook::chapter2::section6::lattice::solve;
use proconio::input;

fn main() {
    input! {
        p1: (isize, isize),
        p2: (isize, isize)
    }
    let ans = solve(p1, p2);
    println!("{ans}");
}
