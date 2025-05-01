use antbook::chapter3::section6::intersection_of_two_prisms::solve;
use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
        p1_list: [(f64, f64); m],
        p2_list: [(f64, f64); n],
    }
    println!("{}", solve(&p1_list, &p2_list));
}
