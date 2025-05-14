pub fn mod_mul(a: &[Vec<usize>], b: &[Vec<usize>], m: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; b[0].len()]; a.len()];
    for i in 0..a.len() {
        for k in 0..b.len() {
            for j in 0..b[0].len() {
                res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % m;
            }
        }
    }
    res
}

pub fn mod_mul_isize(a: &[Vec<isize>], b: &[Vec<isize>], m: isize) -> Vec<Vec<isize>> {
    let mut res = vec![vec![0; b[0].len()]; a.len()];
    for i in 0..a.len() {
        for k in 0..b.len() {
            for j in 0..b[0].len() {
                res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % m;
            }
        }
    }
    res
}

pub fn mod_pow(a: &[Vec<usize>], n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; a.len()]; a.len()];
    for (i, resi) in res.iter_mut().enumerate() {
        resi[i] = 1;
    }
    let mut a = a.to_vec();
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = mod_mul(&res, &a, m);
        }
        a = mod_mul(&a, &a, m);
        n >>= 1;
    }
    res
}

pub fn mod_pow_isize(a: &[Vec<isize>], n: usize, m: isize) -> Vec<Vec<isize>> {
    let mut res = vec![vec![0; a.len()]; a.len()];
    for (i, resi) in res.iter_mut().enumerate() {
        resi[i] = 1;
    }
    let mut a = a.to_vec();
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = mod_mul_isize(&res, &a, m);
        }
        a = mod_mul_isize(&a, &a, m);
        n >>= 1;
    }
    res
}

pub fn transpose(a: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = a[0].len();
    let mut res = vec![vec![0; n]; m];
    for (i, ai) in a.iter().enumerate() {
        for j in 0..m {
            res[j][i] = ai[j];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![5], vec![6]], 100, vec![vec![32]])]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![5], vec![6]], 10, vec![vec![2]])]
    #[case(&[vec![1, 2, 3], vec![4, 5, 6]], &[vec![7, 10, 13], vec![8, 11, 14], vec![9, 12, 15]], 10, vec![vec![0, 8, 6], vec![2, 7, 2]])]
    #[case(&[vec![1, 2, 3], vec![4, 5, 6]], &[vec![7, 10, 13], vec![8, 11, 14], vec![9, 12, 15]], 100, vec![vec![50, 68, 86], vec![22, 67, 12]])]
    #[case(&[vec![1, 2, 3], vec![4, 5, 6]], &[vec![7, 10, 13], vec![8, 11, 14], vec![9, 12, 15]], 1000, vec![vec![50, 68, 86], vec![122, 167, 212]])]
    fn mod_mul_works(
        #[case] a: &[Vec<usize>],
        #[case] b: &[Vec<usize>],
        #[case] m: usize,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert_eq!(expected, mod_mul(a, b, m));
    }

    #[rstest]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![5], vec![6]], 100, vec![vec![32]])]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![5], vec![6]], 10, vec![vec![2]])]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![5], vec![-6]], 10, vec![vec![-4]])]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![-5], vec![-6]], 10, vec![vec![-4]])]
    #[case(&[vec![1, 2, 3]], &[vec![4], vec![-5], vec![-6]], 100, vec![vec![-24]])]
    fn mod_mul_isize_works(
        #[case] a: &[Vec<isize>],
        #[case] b: &[Vec<isize>],
        #[case] m: isize,
        #[case] expected: Vec<Vec<isize>>,
    ) {
        assert_eq!(expected, mod_mul_isize(a, b, m));
    }

    #[rstest]
    #[case(&[vec![1]], 1, 10, vec![vec![1]])]
    #[case(&[vec![1]], 10, 10, vec![vec![1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 0, 10, vec![vec![1, 0], vec![0, 1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 1, 10, vec![vec![1, 1], vec![1, 0]])]
    #[case(&[vec![1, 1], vec![1, 0]], 2, 10, vec![vec![2, 1], vec![1, 1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 3, 10, vec![vec![3, 2], vec![2, 1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 10, 100, vec![vec![89, 55], vec![55, 34]])]
    #[case(&[vec![1, 1], vec![1, 0]], 10, 10, vec![vec![9, 5], vec![5, 4]])]
    fn mod_pow_works(
        #[case] a: &[Vec<usize>],
        #[case] n: usize,
        #[case] m: usize,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert_eq!(expected, mod_pow(a, n, m));
    }

    #[rstest]
    #[case(&[vec![1, 1], vec![1, 0]], 0, 10, vec![vec![1, 0], vec![0, 1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 1, 10, vec![vec![1, 1], vec![1, 0]])]
    #[case(&[vec![1, 1], vec![1, 0]], 2, 10, vec![vec![2, 1], vec![1, 1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 3, 10, vec![vec![3, 2], vec![2, 1]])]
    #[case(&[vec![1, 1], vec![-1, 0]], 3, 10, vec![vec![-1, 0], vec![0, -1]])]
    #[case(&[vec![1, 1], vec![1, 0]], 10, 100, vec![vec![89, 55], vec![55, 34]])]
    #[case(&[vec![1, 1], vec![1, 0]], 10, 10, vec![vec![9, 5], vec![5, 4]])]
    #[case(&[vec![1, 1], vec![-2, 0]], 10, 100, vec![vec![23, -11], vec![22, 34]])]
    #[case(&[vec![1, 1], vec![-2, 0]], 10, 10, vec![vec![3, -1], vec![-8, 4]])]
    fn mod_pow_isize_works(
        #[case] a: &[Vec<isize>],
        #[case] n: usize,
        #[case] m: isize,
        #[case] expected: Vec<Vec<isize>>,
    ) {
        assert_eq!(expected, mod_pow_isize(a, n, m));
    }

    #[rstest]
    #[case(&[vec![1, 2, 3], vec![4, 5, 6]], vec![vec![1, 4], vec![2, 5], vec![3, 6]])]
    #[case(&[vec![1]], vec![vec![1]])]
    #[case(&[vec![1, 2], vec![3, 4]], vec![vec![1, 3], vec![2, 4]])]
    fn transpose_works(#[case] a: &[Vec<usize>], #[case] expected: Vec<Vec<usize>>) {
        assert_eq!(expected, transpose(a));
    }
}
