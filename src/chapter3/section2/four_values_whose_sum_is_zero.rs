use std::collections::HashMap;

pub fn solve(a_list: &[isize], b_list: &[isize], c_list: &[isize], d_list: &[isize]) -> usize {
    let mut map_ab: HashMap<isize, usize> = HashMap::new();
    for a in a_list {
        for b in b_list {
            *map_ab.entry(a + b).or_insert(0) += 1;
        }
    }
    let mut map_cd: HashMap<isize, usize> = HashMap::new();
    for c in c_list {
        for d in d_list {
            *map_cd.entry(c + d).or_insert(0) += 1;
        }
    }
    let mut ans = 0usize;
    for (k, v) in map_ab {
        if map_cd.contains_key(&-k) {
            ans += v * map_cd.get(&-k).unwrap();
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[-45, -41, -36, -36, 26, -32], &[22, -27, 53, 30, -38, -54], &[42, 56, -37, -75, -10, -6], &[-16, 30, 77, -46, 62, 45], 5)]
    #[case(&[1], &[2], &[3], &[4], 0)]
    #[case(&[1, -1], &[2, 2], &[3, 3], &[4, -4], 4)]
    fn it_works(
        #[case] a_list: &[isize],
        #[case] b_list: &[isize],
        #[case] c_list: &[isize],
        #[case] d_list: &[isize],
        #[case] expected: usize,
    ) {
        assert_eq!(expected, solve(a_list, b_list, c_list, d_list));
    }
}
