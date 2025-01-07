use antbook::chapter3::section2::reading_problem::solve;
use proconio::input;

fn main() {
    input! {
        p: usize,
        a_list: [usize; p],
    }
    println!("{}", solve(p, &a_list));
}
