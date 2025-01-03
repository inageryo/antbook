pub fn mod_pow(a: usize, n: usize, m: usize) -> usize {
    let mut res = 1usize;
    let mut a = a;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % m
        }
        a = a * a % m;
        n >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(1, 1, 100, 1)]
    #[case(3, 4, 10, 1)]
    #[case(3, 45, 1000000007, 644897553)]
    fn mod_pow_works(
        #[case] a: usize,
        #[case] n: usize,
        #[case] m: usize,
        #[case] expected: usize,
    ) {
        let result = mod_pow(a, n, m);
        assert_eq!(expected, result);
    }
}
