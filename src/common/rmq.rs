pub struct RMQ {
    n: usize,
    tree: Vec<usize>,
}

impl RMQ {
    pub fn new(n: usize) -> RMQ {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        RMQ {
            n: size,
            tree: vec![10usize.pow(12); 2 * size - 1],
        }
    }

    pub fn update(&mut self, k: usize, a: usize) {
        let mut k = k;
        k += self.n - 1;
        self.tree[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.tree[k] = self.tree[k * 2 + 1].min(self.tree[k * 2 + 2]);
        }
    }

    // get min of [a, b)
    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
        if r <= a || b <= l {
            return usize::MAX;
        }
        if a <= l && r <= b {
            self.tree[k]
        } else {
            let vl = self.query(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.query(a, b, k * 2 + 2, (l + r) / 2, r);
            vl.min(vr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rmq = RMQ::new(8);
        rmq.update(0, 5);
        rmq.update(1, 3);
        rmq.update(2, 7);
        rmq.update(3, 9);
        rmq.update(4, 6);
        rmq.update(5, 4);
        rmq.update(6, 1);
        rmq.update(7, 2);
        assert_eq!(1, rmq.query(0, 7, 0, 0, 8));
        assert_eq!(1, rmq.query(0, 8, 0, 0, 8));
        assert_eq!(4, rmq.query(2, 6, 0, 0, 8));
        assert_eq!(3, rmq.query(0, 6, 0, 0, 8));
        rmq.update(0, 2);
        assert_eq!(2, rmq.query(0, 6, 0, 0, 8));
        rmq = RMQ::new(7);
        rmq.update(0, 5);
        rmq.update(1, 3);
        rmq.update(2, 7);
        rmq.update(3, 9);
        rmq.update(4, 6);
        rmq.update(5, 4);
        rmq.update(6, 1);
        assert_eq!(1, rmq.query(0, 7, 0, 0, 8));
        assert_eq!(3, rmq.query(0, 6, 0, 0, 8));
        rmq.update(3, 1);
        assert_eq!(1, rmq.query(0, 6, 0, 0, 8));
    }
}
