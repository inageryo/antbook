use antbook::chapter3::section2::face_the_right_way::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        cows: Chars,
    }
    let (k, m) = solve(n, &cows);
    println!("{k} {m}",);
}
