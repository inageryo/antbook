use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve(n: usize, graph: &[Vec<(usize, usize)>]) -> usize {
    let inf = 10usize.pow(12);
    let mut pq = BinaryHeap::new();
    let mut d = vec![(inf, inf); n];
    d[0] = (0, inf);
    pq.push(Reverse((0, 0)));
    while !pq.is_empty() {
        let (nd, nv) = pq.pop().unwrap().0;
        if d[nv].1 < nd {
            continue;
        }
        for (dest, cost) in graph[nv].iter() {
            let mut d2 = nd + cost;
            if d[*dest].0 > d2 {
                d2 = d[*dest].0;
                d[*dest] = (nd + cost, d[*dest].1);
                pq.push(Reverse((d[*dest].0, *dest)));
            }
            if d[*dest].1 > d2 {
                d[*dest] = (d[*dest].0, d2);
                pq.push(Reverse((d[*dest].1, *dest)));
            }
        }
    }
    d[n - 1].1
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(4, &[vec![(1, 100)], vec![(0, 100), (2, 250), (3, 200)], vec![(1, 250), (3, 100)], vec![(1, 200), (2, 100)]], 450)]
    #[case(2, &[vec![(1, 100)], vec![(0, 100)]], 300)]
    #[case(4, &[vec![(1, 100), (3, 400)], vec![(0, 100), (2, 250), (3, 200)], vec![(1, 250), (3, 100)], vec![(0, 400), (1, 200), (2, 100)]], 400)]
    fn it_works(#[case] n: usize, #[case] graph: &[Vec<(usize, usize)>], #[case] expected: usize) {
        assert_eq!(expected, solve(n, graph));
    }
}
