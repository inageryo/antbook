use crate::common::gcd::extended_gcd;

pub fn solve(a: usize, b: usize) -> String {
    let x = &mut 0;
    let y = &mut 0;
    let gcd = extended_gcd(a as isize, b as isize, x, y);
    if gcd != 1 {
        (-1).to_string()
    } else if *x >= 0 && *y >= 0 {
        format!("{x} {y} 0 0")
    } else if *x >= 0 && *y < 0 {
        format!("{x} 0 0 {}", y.abs())
    } else if *x < 0 && *y >= 0 {
        format!("0 {y} {} 0", x.abs())
    } else {
        format!("0 0 {} {}", x.abs(), y.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(4, 11, "3 0 0 1")]
    #[case(4, 12, "-1")]
    #[case(11, 3, "0 4 1 0")]
    #[case(1, 3, "1 0 0 0")]
    fn it_works(#[case] a: usize, #[case] b: usize, #[case] expected: String) {
        assert_eq!(expected, solve(a, b));
    }
}
