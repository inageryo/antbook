use antbook::chapter3::section4::traveling_by_stagecoach::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: Usize1,
        b: Usize1,
        t_list: [f64; n],
        d_list: [[f64; m]; m]
    }
    if let Some(ans) = solve(n, m, a, b, &t_list, &d_list) {
        println!("{}", ans);
    } else {
        println!("Impossible")
    }
}
