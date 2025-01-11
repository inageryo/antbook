pub struct BIT {
    bit: Vec<isize>,
}

impl BIT {
    pub fn new(n: usize) -> BIT {
        BIT {
            bit: vec![0; n + 1],
        }
    }

    pub fn sum(&self, i: usize) -> isize {
        let mut s = 0;
        let mut i = i as isize;
        while i > 0 {
            s += self.bit[i as usize];
            i -= i & -i;
        }
        s
    }

    pub fn add(&mut self, i: usize, x: isize) {
        let mut i = i as isize;
        while i < self.bit.len() as isize {
            self.bit[i as usize] += x;
            i += i & -i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bit = BIT::new(8);
        bit.add(1, 5);
        bit.add(2, 3);
        bit.add(3, 7);
        bit.add(4, 9);
        bit.add(5, 6);
        bit.add(6, 4);
        bit.add(7, 1);
        bit.add(8, 2);
        assert_eq!(0, bit.sum(0));
        assert_eq!(5, bit.sum(1));
        assert_eq!(35, bit.sum(7));
        assert_eq!(37, bit.sum(8));
        bit.add(1, -3);
        assert_eq!(5, bit.sum(2));
        assert_eq!(21, bit.sum(4));
        assert_eq!(34, bit.sum(8));
        bit = BIT::new(1);
        bit.add(1, 10);
        assert_eq!(10, bit.sum(1));
    }
}
