use antbook::chapter3::section4::domino::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        matrix: [Chars; n]
    }
    println!("{}", solve(n, m, &matrix));
}
