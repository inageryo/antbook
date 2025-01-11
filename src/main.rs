use antbook::chapter3::section3::crane::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        c: usize,
        l_list: [usize; n],
        s_list: [Usize1; c],
        c_list: [usize; c],
    }
    for e in solve(n, c, &l_list, &s_list, &c_list) {
        println!("{} {}", e.0, e.1);
    }
}
