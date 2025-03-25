use antbook::chapter3::section5::farm_tour::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        roads: [(Usize1, Usize1, usize); m],
    }
    println!("{}", solve(n, &roads));
}
