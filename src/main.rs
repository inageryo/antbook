use antbook::chapter2::maze::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut maze: [Chars; n],
    }
    let ans = solve(n, m, &maze);
    println!("{ans}");
}
