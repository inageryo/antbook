use std::collections::BinaryHeap;

pub fn solve(n: usize, l: usize, p: usize, a_list: &[usize], b_list: &[usize]) -> isize {
    let mut pq = BinaryHeap::new();
    pq.push(p);
    let mut current = 0;
    let mut idx = 0;
    let mut count = 0;
    while !pq.is_empty() {
        let np = pq.pop().unwrap();
        current += np;
        count += 1;
        if current >= l {
            return count - 1;
        }
        for i in idx..n {
            if a_list[i] <= current {
                pq.push(b_list[i]);
            } else {
                idx = i;
                break;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, 25, 10, &[10, 14, 20, 21], &[10, 5, 2, 4], 2)]
    #[case(7, 25, 10, &[2, 3, 5, 10, 14, 20, 22], &[6, 2, 8, 5, 10, 2, 3], 2)]
    #[case(4, 25, 25, &[10, 14, 20, 21], &[10, 5, 2, 4], 0)]
    #[case(4, 25, 24, &[10, 14, 20, 21], &[10, 5, 2, 4], 1)]
    #[case(4, 25, 9, &[10, 14, 20, 21], &[10, 5, 2, 4], -1)]
    fn it_works(
        #[case] n: usize,
        #[case] l: usize,
        #[case] p: usize,
        #[case] a_list: &[usize],
        #[case] b_list: &[usize],
        #[case] expected: isize,
    ) {
        assert_eq!(expected, solve(n, l, p, a_list, b_list));
    }
}
