use antbook::chapter3::section3::kth_number::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_list: [isize; n],
        q_list: [(Usize1, Usize1, usize); m]
    }
    for x in solve(n, &a_list, &q_list) {
        println!("{x}");
    }
}
