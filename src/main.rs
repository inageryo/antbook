use antbook::chapter3::section2::count_realm::solve;
use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        mut x1: [usize; n],
        mut x2: [usize; n],
        mut y1: [usize; n],
        mut y2: [usize; n],
    }
    println!("{}", solve(w, h, n, &mut x1, &mut x2, &mut y1, &mut y2));
}
