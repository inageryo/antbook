pub fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        return n;
    }
    gcd(m, n % m)
}

pub fn extended_gcd(a: isize, b: isize, x: &mut isize, y: &mut isize) -> isize {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let d = extended_gcd(b, a % b, y, x);
    *y -= a / b * *x;
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 2, 2)]
    #[case(2, 5, 1)]
    #[case(1, 5, 1)]
    #[case(26, 32, 2)]
    #[case(29, 32, 1)]
    #[case(28, 32, 4)]
    fn gcd_works(#[case] n: usize, #[case] m: usize, #[case] expected: usize) {
        assert_eq!(expected, gcd(n, m));
    }

    #[rstest]
    #[case(1, 1, 1, 0, 1)]
    #[case(2, 2, 2, 0, 1)]
    #[case(2, 5, 1, -2, 1)]
    #[case(1, 5, 1, 1, 0)]
    #[case(26, 32, 2, 5, -4)]
    #[case(29, 32, 1, -11, 10)]
    #[case(28, 32, 4, -1, 1)]
    #[case(100, 13, 1, 3, -23)]
    fn extended_gcd_works(
        #[case] n: isize,
        #[case] m: isize,
        #[case] expected: isize,
        #[case] expected_x: isize,
        #[case] expected_y: isize,
    ) {
        let x = &mut 0;
        let y = &mut 0;
        assert_eq!(expected, extended_gcd(n, m, x, y));
        assert_eq!(expected_x, *x);
        assert_eq!(expected_y, *y);
    }
}
