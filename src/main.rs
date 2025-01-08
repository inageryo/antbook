use antbook::chapter3::section2::flip_tile::solve;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
        tiles: [[usize; n]; m],
    }
    if let Some(ans) = solve(m, n, &tiles) {
        ans.iter().for_each(|a| println!("{}", a.iter().join(" ")));
    } else {
        println!("IMPOSSIBLE");
    }
}
