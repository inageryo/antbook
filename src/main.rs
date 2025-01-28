use antbook::chapter3::section5::minimum_cost::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        f: usize,
        e: usize,
        network: [(usize, usize, usize, usize); e]
    }
    if let Some(ans) = solve(n, s, t, f, &network) {
        println!("{}", ans);
    } else {
        println!("impossible");
    }
}
