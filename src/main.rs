use antbook::chapter1::triangles::solve;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_list: [usize; n],
    }
    let ans = solve(n, &mut a_list);
    println!("{ans}");
}
