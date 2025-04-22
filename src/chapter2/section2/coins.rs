pub fn solve(
    c1: usize,
    c5: usize,
    c10: usize,
    c50: usize,
    c100: usize,
    c500: usize,
    a: usize,
) -> usize {
    let mut ans = 0usize;
    let mut a = a;
    for (v, c) in [
        (500, c500),
        (100, c100),
        (50, c50),
        (10, c10),
        (5, c5),
        (1, c1),
    ] {
        let ac = (a / v).min(c);
        ans += ac;
        a -= ac * v;
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    struct Coin {
        c1: usize,
        c5: usize,
        c10: usize,
        c50: usize,
        c100: usize,
        c500: usize,
    }

    #[rstest]
    #[case(Coin {c1: 3, c5: 2, c10: 1, c50: 3, c100: 0, c500: 2}, 620, 6)]
    #[case(Coin {c1: 3, c5: 2, c10: 1, c50: 3, c100: 1, c500: 2}, 620, 5)]
    #[case(Coin {c1: 500, c5: 200, c10: 100, c50: 300, c100: 10, c500: 2}, 500, 1)]
    fn it_works(#[case] coin: Coin, #[case] a: usize, #[case] expected: usize) {
        let result = solve(
            coin.c1, coin.c5, coin.c10, coin.c50, coin.c100, coin.c500, a,
        );
        assert_eq!(expected, result);
    }
}
