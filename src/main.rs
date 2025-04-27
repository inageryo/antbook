use antbook::chapter3::section6::white_bird::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: f64,
        (x, y): (f64, f64),
        obstacle_list: [(f64, f64, f64, f64); n]
    }
    if solve(v, x, y, &obstacle_list) {
        println!("Yes");
    } else {
        println!("No");
    }
}
