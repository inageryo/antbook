use antbook::chapter3::section5::evacuation_plan_2::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        bills: [(isize, isize, usize); n],
        shelters: [(isize, isize, usize); m],
        plan: [[usize; m]; n]
    }
    if let Some(op) = solve(n, m, &bills, &shelters, &plan) {
        println!("SUBOPTIMAL");
        for r in op.iter() {
            for (i, c) in r.iter().enumerate() {
                if i < r.len() - 1 {
                    print!("{} ", c);
                } else {
                    println!("{}", c);
                }
            }
        }
    } else {
        println!("OPTIONAL");
    }
}
