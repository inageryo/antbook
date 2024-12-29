use antbook::chapter2::lake_counting::solve;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut garden: [Chars; n],
    }
    let ans = solve(n, m, &mut garden);
    println!("{ans}");
}
