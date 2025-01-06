use antbook::chapter3::section2::two_pointer::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        mut items: [usize; n],
    }
    println!("{}", solve(n, s, &items));
}
