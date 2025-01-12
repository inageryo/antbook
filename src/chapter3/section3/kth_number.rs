use itertools::Itertools;

pub fn solve(n: usize, a_list: &[isize], q_list: &[(usize, usize, usize)]) -> Vec<isize> {
    let bucket_size = (n as f64).sqrt() as usize;
    let mut bucket_list = vec![];
    for i in (0..n).step_by(bucket_size) {
        let b = &a_list[i..(i + bucket_size).min(n)];
        let mut b = b.to_vec();
        b.sort();
        bucket_list.push(b);
    }
    let sorted_a_list = a_list.iter().copied().sorted().collect::<Vec<isize>>();
    let mut ans = vec![];
    for (l, r, k) in q_list {
        let mut lb = -1;
        let mut ub = (n - 1) as isize;
        while ub - lb > 1 {
            let mid = (lb + ub) / 2;
            let x = sorted_a_list[mid as usize];
            let mut tl = *l;
            let mut tr = *r + 1;
            let mut count = 0;
            while tl < tr && tl % bucket_size != 0 {
                if a_list[tl] <= x {
                    count += 1;
                }
                tl += 1;
            }
            while tl < tr && tr % bucket_size != 0 {
                tr -= 1;
                if a_list[tr] <= x {
                    count += 1;
                }
            }
            while tl < tr {
                let bucket_index = tl / bucket_size;
                count += bucket_list[bucket_index].partition_point(|&e| e <= x);
                tl += bucket_size;
            }
            if count >= *k {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        ans.push(sorted_a_list[ub as usize]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(7, &[1, 5, 2, 6, 3, 7, 4], &[(1, 4, 3), (3, 3, 1), (0, 6, 3)], vec![5, 6, 3])]
    #[case(7, &[1, 5, 2, 5, 7, 7, 4], &[(1, 4, 3), (2, 3, 1), (0, 6, 3)], vec![5, 2, 4])]
    #[case(7, &[1, 5, 2, 6, 3, 7, 4], &[(3, 3, 1), (2, 2, 1), (1, 1, 1)], vec![6, 2, 5])]
    #[case(7, &[1, 50, -2, -6, 30, -7, 4], &[(3, 5, 1), (2, 2, 1), (0, 6, 3), (0, 6, 6)], vec![-7, -2, -2, 30])]
    fn it_works(
        #[case] n: usize,
        #[case] a_list: &[isize],
        #[case] q_list: &[(usize, usize, usize)],
        #[case] expected: Vec<isize>,
    ) {
        assert_eq!(expected, solve(n, a_list, q_list));
    }
}
