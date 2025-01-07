use itertools::Itertools;

pub fn solve(n: usize, cows: &[char]) -> (usize, usize) {
    // fixme: too slow
    if cows.iter().unique().count() == 1 && cows[0] == 'F' {
        return (0, 0);
    }
    let mut k = 0;
    let mut m = usize::MAX;
    for i in 1..n {
        let mut cows = cows.to_owned();
        let mut p = 0;
        let mut count = 0usize;
        while p < n {
            if cows[p] == 'F' {
                p += 1;
            } else if p + i - 1 < n {
                for j in 0..i {
                    if cows[p + j] == 'F' {
                        cows[p + j] = 'B';
                    } else {
                        cows[p + j] = 'F';
                    }
                }
                count += 1;
            } else {
                break;
            }
        }
        if p == n && count < m {
            m = count;
            k = i;
        }
    }
    (k, m)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(7, &['B', 'B', 'F', 'B', 'F', 'B', 'B'], (3, 3))]
    #[case(5, &['F', 'F', 'F', 'F', 'F'], (0, 0))]
    #[case(5, &['B', 'B', 'B', 'B', 'B'], (1, 5))]
    #[case(6, &['F', 'F', 'F', 'F', 'F', 'B'], (1, 1))]
    #[case(6, &['F', 'F', 'F', 'F', 'B', 'B'], (2, 1))]
    #[case(6, &['F', 'B', 'F', 'B', 'F', 'B'], (1, 3))]
    fn it_works(#[case] n: usize, #[case] cows: &[char], #[case] expected: (usize, usize)) {
        assert_eq!(expected, solve(n, cows));
    }
}
