use std::collections::HashMap;

pub fn solve(
    n: usize,
    m: usize,
    graph_m: &[HashMap<usize, usize>],
    graph_f: &[HashMap<usize, usize>],
) -> usize {
    let mut ans = 0;
    for e in graph_m.iter() {
        ans += e.values().sum::<usize>();
    }
    for e in graph_f.iter() {
        ans += e.values().sum::<usize>();
    }
    (n + m) * 10_000 - ans / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let graph_m = vec![
            HashMap::from([(0, 6592), (1, 3063)]),
            HashMap::from([(3, 4583)]),
            HashMap::from([(2, 781)]),
            HashMap::from([(3, 4975)]),
            HashMap::from([(2, 2104), (3, 6831)]),
        ];
        let graph_f = vec![
            HashMap::from([(0, 6592)]),
            HashMap::from([(0, 3063)]),
            HashMap::from([(2, 781), (4, 2104)]),
            HashMap::from([(1, 4583), (3, 4975), (4, 6831)]),
            HashMap::new(),
        ];
        assert_eq!(71071, solve(5, 5, &graph_m, &graph_f));
    }
}
