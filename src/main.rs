use antbook::chapter3::section7::watering_plants::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        circles: [(f64, f64, f64); n]
    }
    println!("{}", solve(&circles));
}
