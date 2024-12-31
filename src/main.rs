use antbook::chapter2::section3::partial_sum_limited::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_list: [isize; n],
        m_list: [isize; n],
        k: usize,
    }
    let ans = solve(n, &a_list, &m_list, k);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
