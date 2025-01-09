use antbook::chapter3::section2::four_values_whose_sum_is_zero::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_list: [isize; n],
        b_list: [isize; n],
        c_list: [isize; n],
        d_list: [isize; n],
    }
    println!("{}", solve(&a_list, &b_list, &c_list, &d_list));
}
