use antbook::chapter3::section5::dining::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        f: usize,
        d: usize,
        fc: usize,
        dc: usize,
        food_preference: [(Usize1, Usize1); fc],
        drink_preference: [(Usize1, Usize1); dc],
    }
    println!("{}", solve(n, f, d, &food_preference, &drink_preference));
}
