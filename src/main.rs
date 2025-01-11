use antbook::chapter3::section3::simple_problem_with_integers::solve;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a_list: [usize; n],
        q: usize,
        t_list: [char; q],
        l_list: [Usize1; q],
        r_list: [Usize1; q],
        x_list: [usize; q],
    }
    for x in solve(n, &a_list, q, &t_list, &l_list, &r_list, &x_list) {
        println!("{x}");
    }
}
