use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(p: usize, a_list: &[usize]) -> usize {
    let total = a_list.iter().unique().count();
    let mut ans = usize::MAX;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(a_list[0], 1);
    loop {
        if map.len() < total {
            p2 += 1;
            if p2 >= p {
                break;
            }
            *map.entry(a_list[p2]).or_insert(0) += 1;
        } else {
            ans = ans.min(p2 - p1 + 1);
            map.entry(a_list[p1]).and_modify(|v| *v -= 1);
            if *map.get(&a_list[p1]).unwrap() == 0 {
                map.remove(&a_list[p1]);
            }
            p1 += 1;
            if p1 > p2 {
                return ans;
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
    #[case(5, &[1, 8, 8, 8, 1], 2)]
    #[case(5, &[1, 1, 1, 1, 1], 1)]
    #[case(5, &[1, 4, 3, 2, 5], 5)]
    #[case(5, &[1, 4, 1, 1, 5], 4)]
    #[case(5, &[3, 4, 1, 1, 5], 5)]
    #[case(1, &[1], 1)]
    #[case(15, &[1, 5, 2, 3, 5, 3, 4, 1, 2, 3, 2, 5, 1, 2, 1], 5)]
    #[case(15, &[1, 5, 2, 3, 3, 3, 4, 1, 2, 3, 2, 5, 1, 2, 1], 6)]
    fn it_works(#[case] p: usize, #[case] a_list: &[usize], #[case] expected: usize) {
        assert_eq!(expected, solve(p, a_list));
    }
}
