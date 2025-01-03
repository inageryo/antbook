pub fn solve(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut is_prime = vec![true; n + 1];
    let mut count = 0usize;
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            count += 1;
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(11, 5)]
    #[case(1_000_000, 78498)]
    fn it_works(#[case] n: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(n));
    }
}
