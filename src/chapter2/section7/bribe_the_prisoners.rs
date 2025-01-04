use std::collections::{HashMap, HashSet};

pub fn solve(p: usize, a_list: &[usize]) -> usize {
    let a_set: HashSet<usize> = HashSet::from_iter(a_list.iter().copied());
    dfs(0, p - 1, &a_set, &mut HashMap::new())
}

fn dfs(
    start: usize,
    end: usize,
    a_set: &HashSet<usize>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if start > end {
        return 0;
    }
    if memo.contains_key(&(start, end)) {
        return *memo.get(&(start, end)).unwrap();
    }
    let mut ret = usize::MAX;
    for i in start..=end {
        if a_set.contains(&i) {
            ret = ret
                .min(dfs(start, i - 1, a_set, memo) + dfs(i + 1, end, a_set, memo) + end - start);
        }
    }
    if ret == usize::MAX {
        ret = 0;
    }
    memo.insert((start, end), ret);
    ret
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(8, &[2], 7)]
    #[case(20, &[3], 19)]
    #[case(20, &[2, 5], 23)]
    #[case(20, &[2, 5, 13], 35)]
    #[case(10000, &[1, 2, 5, 13, 43, 456, 6574, 9999], 20510)]
    fn it_works(#[case] p: usize, #[case] a_list: &[usize], #[case] expected: usize) {
        assert_eq!(expected, solve(p, a_list));
    }
}
