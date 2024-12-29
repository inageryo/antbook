use std::collections::HashSet;

pub fn solve(m: usize, k_list: Vec<usize>) -> String {
    let mut s = HashSet::new();
    for k1 in k_list.iter() {
        for k2 in k_list.iter() {
            s.insert(k1 + k2);
        }
    }
    for e in s.iter() {
        if m > *e && s.contains(&(m - *e)) {
            return "Yes".to_string();
        }
    }
    "No".to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(10, vec![1, 3, 5], "Yes")]
    #[case(9, vec![1, 3, 5], "No")]
    #[case(4, vec![1], "Yes")]
    #[case(3, vec![1], "No")]
    fn it_works(#[case] m: usize, #[case] k_list: Vec<usize>, #[case] expected: String) {
        let result = solve(m, k_list);
        assert_eq!(expected, result);
    }
}
