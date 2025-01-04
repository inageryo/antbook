pub fn solve(n: usize, matrix: &mut [Vec<u8>]) -> usize {
    let mut v = matrix
        .iter()
        .map(|e| {
            for i in (0..n).rev() {
                if e[i] == 1 {
                    return i;
                }
            }
            0
        })
        .collect::<Vec<usize>>();
    let mut ans = 0usize;
    for i in 0..n {
        if v[i] <= i {
            continue;
        }
        for j in i + 1..n {
            if v[j] <= i {
                for k in (i..j).rev() {
                    matrix.swap(k + 1, k);
                    v.swap(k + 1, k);
                    ans += 1;
                }
                break;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(2, &mut [vec![1, 0], vec![1, 1]], 0)]
    #[case(3, &mut [vec![0, 0, 1], vec![1, 0, 0], vec![0, 1, 0]], 2)]
    #[case(4, &mut [vec![1, 1, 1, 0], vec![1, 1, 0, 0], vec![1, 1, 0, 0], vec![1, 0, 0, 0]], 4)]
    #[case(4, &mut [vec![1, 1, 1, 0], vec![0, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 0, 0]], 2)]
    fn it_works(#[case] n: usize, #[case] matrix: &mut [Vec<u8>], #[case] expected: usize) {
        assert_eq!(expected, solve(n, matrix));
    }
}
