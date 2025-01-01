pub struct UnionFind {
    parents: Vec<isize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: vec![-1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> isize {
        if self.parents[x] < 0 {
            x as isize
        } else {
            self.parents[x] = self.find(self.parents[x] as usize);
            self.parents[x]
        }
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let mut xi = self.find(x);
        let mut yi = self.find(y);
        if xi == yi {
            return;
        }
        if self.parents[xi as usize] > self.parents[yi as usize] {
            (xi, yi) = (yi, xi);
        }
        self.parents[xi as usize] += self.parents[yi as usize];
        self.parents[yi as usize] = xi;
    }
    pub fn size(&mut self, x: usize) -> usize {
        let xi = self.find(x) as usize;
        -self.parents[xi] as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert_eq!(1, uf.size(i));
        }
        assert_ne!(uf.find(0), uf.find(1));
        uf.union(0, 1);
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        uf.union(2, 3);
        assert_eq!(uf.find(2), uf.find(3));
        assert_ne!(uf.find(0), uf.find(2));
        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        assert_eq!(2, uf.size(2));
        assert_eq!(2, uf.size(3));
        uf.union(0, 4);
        assert_eq!(uf.find(0), uf.find(4));
        assert_ne!(uf.find(0), uf.find(2));
        assert_eq!(3, uf.size(0));
        assert_eq!(3, uf.size(4));
        assert_eq!(2, uf.size(2));
        uf.union(4, 3);
        assert_eq!(uf.find(0), uf.find(2));
        for i in 0..5 {
            assert_eq!(5, uf.size(i));
        }
    }
}
