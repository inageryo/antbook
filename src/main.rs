use antbook::chapter3::section6::jack_straws::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        p_list: [(isize, isize); n],
        q_list: [(isize, isize); n],
        m: usize,
        ab_list: [(Usize1, Usize1); m]
    }
    for b in solve(n, &p_list, &q_list, &ab_list) {
        if b {
            println!("CONNECTED")
        } else {
            println!("NOT CONNECTED")
        }
    }
}
