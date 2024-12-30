pub fn solve(s: String) -> String {
    let mut s = s;
    let mut ans = String::new();
    while !s.is_empty() {
        let reversed: String = s.chars().rev().collect();
        let c = if s <= reversed {
            s.remove(0)
        } else {
            s.pop().unwrap()
        };
        ans += &c.to_string();
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("ACDBCB".to_string(), "ABCBCD".to_string())]
    #[case("ACDBCD".to_string(), "ACDBCD".to_string())]
    #[case("ABCBA".to_string(), "AABBC".to_string())]
    fn it_works(#[case] s: String, #[case] expected: String) {
        assert_eq!(solve(s), expected);
    }
}
