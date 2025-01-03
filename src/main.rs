use antbook::chapter2::section5::layout::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        ml: usize,
        md: usize,
        l_list: [(Usize1, Usize1, usize); ml],
        d_list: [(Usize1, Usize1, usize); md],
    }
    let ans = solve(n, &l_list, &d_list);
    println!("{ans}");
}
