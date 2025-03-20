use antbook::chapter3::section5::dual_core_cpu::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        cost_list: [(usize, usize); n],
        exchange_cost_list: [(usize, usize, usize); m],
    }
    println!("{}", solve(n, &cost_list, &exchange_cost_list));
}
