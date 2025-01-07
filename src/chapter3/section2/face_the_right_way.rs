use itertools::Itertools;

pub fn solve(n: usize, cows: &[char]) -> (usize, usize) {
    if cows.iter().unique().count() == 1 && cows[0] == 'F' {
        return (0, 0);
    }
    let mut k = 0;
    let mut m = usize::MAX;
    for i in 1..n {
        if let Some(count) = get_count(i, n, cows) {
            if count < m {
                m = count;
                k = i;
            }
        }
    }
    (k, m)
}

fn get_count(k: usize, n: usize, cows: &[char]) -> Option<usize> {
    let mut ans = 0;
    let mut s = 0usize;
    let mut f = vec![0; n - k + 1];
    for i in 0..n - k + 1 {
        if s % 2 == 0 && cows[i] == 'B' || s % 2 == 1 && cows[i] == 'F' {
            f[i] = 1;
            ans += 1;
        }
        s += f[i];
        if i + 1 >= k {
            s -= f[i + 1 - k];
        }
    }
    for i in n - k + 1..n {
        if s % 2 == 0 && cows[i] == 'B' || s % 2 == 1 && cows[i] == 'F' {
            return None;
        }
        if i + 1 >= k {
            s -= f[i + 1 - k];
        }
    }
    Some(ans)
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
