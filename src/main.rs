use antbook::chapter1::ants::solve;
use proconio::input;

fn main() {
    input! {
        l: usize,
        n: usize,
        a_list: [usize; n],
    }
    let (min, max) = solve(l, a_list);
    println!("{min} {max}");
}
