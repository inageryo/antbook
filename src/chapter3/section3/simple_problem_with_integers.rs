use crate::common::bit::BIT;

pub fn solve(
    n: usize,
    a_list: &[usize],
    q: usize,
    t_list: &[char],
    l_list: &[usize],
    r_list: &[usize],
    x_list: &[usize],
) -> Vec<isize> {
    let mut bit0 = BIT::new(n);
    let mut bit1 = BIT::new(n);
    for (i, e) in a_list.iter().enumerate() {
        let i = i + 1;
        bit0.add(i, -((*e * (i - 1)) as isize));
        bit1.add(i, *e as isize);
        bit0.add(i + 1, (*e * i) as isize);
        bit1.add(i + 1, -(*e as isize));
    }
    let mut ans = vec![];
    for i in 0..q 
    {
        let l = l_list[i] + 1;
        let r = r_list[i] + 1;
        let x = x_list[i] as isize;
        if t_list[i] == 'C' {
            bit0.add(l, -x * (l - 1) as isize);
            bit1.add(l, x);
            bit0.add(r + 1, x * r as isize);
            bit1.add(r + 1, -x);
        } else {
            ans.push(
                bit1.sum(r) * r as isize + bit0.sum(r)
                    - (bit1.sum(l - 1) * (l - 1) as isize + bit0.sum(l - 1)),
            );
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    struct IntParameter {
        n: usize,
        q: usize,
    }

    #[rstest]
    #[case(IntParameter {n: 4, q: 2}, &[5, 3, 7, 9], &['C', 'Q'], &[0, 0], &[2, 3], &[1, 0], vec![27])]
    #[case(IntParameter {n: 4, q: 1}, &[5, 3, 7, 9], &['C'], &[0], &[2], &[1], vec![])]
    #[case(IntParameter {n: 1, q: 3}, &[5], &['Q', 'C', 'Q'], &[0, 0, 0], &[0, 0, 0], &[0, 1, 0], vec![5, 6])]
    #[case(IntParameter {n: 8, q: 3}, &[5, 3, 7, 9, 6, 4, 1, 2], &['Q', 'C', 'Q'], &[0, 0, 0], &[7, 4, 7], &[0, 1, 0], vec![37, 42])]
    fn it_works(
        #[case] int_parameter: IntParameter,
        #[case] a_list: &[usize],
        #[case] t_list: &[char],
        #[case] l_list: &[usize],
        #[case] r_list: &[usize],
        #[case] x_list: &[usize],
        #[case] expected: Vec<isize>,
    ) {
        assert_eq!(
            expected,
            solve(
                int_parameter.n,
                a_list,
                int_parameter.q,
                t_list,
                l_list,
                r_list,
                x_list
            )
        );
    }
}
