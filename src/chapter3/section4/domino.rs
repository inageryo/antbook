const M: usize = 998_244_353;
pub fn solve(n: usize, m: usize, matrix: &[Vec<char>]) -> usize {
    let size = 1 << m;
    let mut dp = vec![vec![0; size]; n * m + 1];
    dp[0][0] = 1;
    if matrix
        .iter()
        .map(|x| x.iter().filter(|&&x1| x1 == 'X').count())
        .sum::<usize>()
        == n * m
    {
        return 0;
    }
    for i in 0..n {
        for j in 0..m {
            for s in 0..size {
                if s & (1 << (m - 1)) == 1 << (m - 1) || matrix[i][j] == 'X' {
                    dp[i * m + j + 1][(s << 1) & (size - 1)] =
                        (dp[i * m + j + 1][(s << 1) & (size - 1)] + dp[i * m + j][s]) % M;
                } else {
                    if i < n - 1 && matrix[i + 1][j] == '.' {
                        dp[i * m + j + 1][(s << 1) & (size - 1) | 1] =
                            (dp[i * m + j + 1][(s << 1) & (size - 1) | 1] + dp[i * m + j][s]) % M;
                    }
                    if j < m - 1 && matrix[i][j + 1] == '.' {
                        dp[i * m + j + 1][(s << 1) & (size - 1) | (1 << (m - 1))] = (dp
                            [i * m + j + 1][(s << 1) & (size - 1) | (1 << (m - 1))]
                            + dp[i * m + j][s])
                            % M;
                    }
                }
            }
        }
    }
    dp[n * m][0]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 3, &[vec!['.', '.', '.'], vec!['.', 'X', '.'], vec!['.', '.', '.']], 2)]
    #[case(1, 1, &[vec!['.']], 0)]
    #[case(1, 1, &[vec!['X']], 0)]
    #[case(1, 2, &[vec!['.', '.']], 1)]
    #[case(1, 2, &[vec!['.', 'X']], 0)]
    #[case(1, 2, &[vec!['X', '.']], 0)]
    #[case(1, 2, &[vec!['X', 'X']], 0)]
    #[case(3, 1, &[vec!['X'], vec!['.'], vec!['.']], 1)]
    #[case(3, 1, &[vec!['.'], vec!['.'], vec!['X']], 1)]
    #[case(3, 1, &[vec!['.'], vec!['X'], vec!['.']], 0)]
    #[case(3, 1, &[vec!['.'], vec!['.'], vec!['.']], 0)]
    #[case(3, 1, &[vec!['X'], vec!['X'], vec!['X']], 0)]
    #[case(3, 2, &[vec!['.', '.'], vec!['.', '.'], vec!['.', '.']], 3)]
    fn it_works(
        #[case] n: usize,
        #[case] m: usize,
        #[case] matrix: &[Vec<char>],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, m, matrix));
    }
}
