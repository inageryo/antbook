use antbook::chapter3::section7::no_cheating::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        m: usize,
        n: usize,
        seats: [Chars; m]
    }
    println!("{}", solve(m, n, &seats));
}
