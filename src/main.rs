use antbook::chapter3::section5::evacuation::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: usize,
        y: usize,
        positions: [Chars; x]
    }
    if let Some(ans) = solve(x, y, &positions) {
        println!("{}", ans);
    } else {
        println!("impossible");
    }
}
