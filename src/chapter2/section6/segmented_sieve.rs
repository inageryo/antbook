pub fn solve(a: usize, b: usize) -> usize {
    let b_sqrt = (b as f64).sqrt() as usize;
    let mut is_prime1 = vec![true; b_sqrt + 1];
    let mut is_prime2 = vec![true; b - a];
    is_prime1[0] = false;
    is_prime1[1] = false;
    for i in 2..=b_sqrt {
        if is_prime1[i] {
            for j in (i * i..=b_sqrt).step_by(i) {
                is_prime1[j] = false;
            }
            for j in ((i * a.div_ceil(i) - a)..(b - a)).step_by(i) {
                is_prime2[j] = false;
            }
        }
    }
    is_prime2.iter().filter(|e| **e).count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(22, 37, 3)]
    #[case(22, 47, 6)]
    #[case(22, 49, 7)]
    #[case(22801763489, 22801787297, 1000)]
    fn it_works(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        assert_eq!(expected, solve(a, b));
    }
}
