pub fn solve(n: usize, w_list: &[usize], v_list: &[usize], w_limit: usize) -> usize {
    if n == 1 {
        return if w_list[0] <= w_limit { v_list[0] } else { 0 };
    }
    let n1 = n / 2;
    let w_list1 = &w_list[..n1];
    let v_list1 = &v_list[..n1];
    let n2 = n - n1;
    let w_list2 = &w_list[n1..];
    let v_list2 = &v_list[n1..];
    let mut w_v_list = vec![];
    for bit in 0..(1 << n1) {
        let mut w = 0usize;
        let mut v = 0usize;
        for j in 0..n1 {
            if bit & (1 << j) > 0 {
                w += w_list1[j];
                v += v_list1[j];
            }
        }
        w_v_list.push((w, v));
    }
    w_v_list.sort();
    let mut w_v_max_list = vec![];
    let mut v_max = 0usize;
    for (w, v) in w_v_list {
        if w <= w_limit {
            v_max = v_max.max(v);
            w_v_max_list.push((w, v_max));
        }
    }
    let mut ans = v_max;
    for bit in 0..(1 << n2) {
        let mut w = 0usize;
        let mut v = 0usize;
        for j in 0..n1 {
            if bit & (1 << j) > 0 {
                w += w_list2[j];
                v += v_list2[j];
            }
        }
        let idx = w_v_max_list.partition_point(|&x| x.0 + w <= w_limit);
        if idx > 0 {
            ans = ans.max(v + w_v_max_list[idx - 1].1);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, &[2, 1, 3, 2], &[3, 2, 4, 2], 5, 7)]
    #[case(2, &[2, 4], &[5, 7], 1, 0)]
    #[case(2, &[2, 4], &[5, 7], 2, 5)]
    #[case(2, &[2, 4], &[5, 7], 5, 7)]
    #[case(2, &[2, 4], &[5, 7], 6, 12)]
    #[case(1, &[1_000_000_000_000], &[10_000_000_000_000], 100_000_000_000_000, 10_000_000_000_000)]
    #[case(1, &[1_000_000_000_000], &[10_000_000_000_000], 100_000_000_000, 0)]
    fn it_works(
        #[case] n: usize,
        #[case] w_list: &[usize],
        #[case] v_list: &[usize],
        #[case] w_limit: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(n, w_list, v_list, w_limit));
    }
}
