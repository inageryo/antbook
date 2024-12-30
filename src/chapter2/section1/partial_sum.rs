pub fn solve(a_list: &Vec<isize>, k: isize) -> String {
    let mut found = false;
    dfs(0, 0, a_list, k, &mut found);
    if found {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn dfs(current_index: usize, current_sum: isize, a_list: &Vec<isize>, k: isize, found: &mut bool) {
    if current_sum == k {
        *found = true;
        return;
    }
    if current_index >= a_list.len() || current_sum > k || *found {
        return;
    }
    dfs(
        current_index + 1,
        current_sum + a_list[current_index],
        a_list,
        k,
        found,
    );
    dfs(current_index + 1, current_sum, a_list, k, found);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![1, 2, 4, 7], 13, "Yes")]
    #[case(vec![1, 2, 4, 7], 15, "No")]
    #[case(vec![1, 2, 4, 7], 0, "Yes")]
    #[case(vec![1, 2, 4, 7], -1, "No")]
    #[case(vec![-1, -2, -4, 7], 0, "Yes")]
    fn it_works(#[case] a_list: Vec<isize>, #[case] k: isize, #[case] expected: String) {
        let result = solve(&a_list, k);
        assert_eq!(expected, result);
    }
}
