use std::ops::{AddAssign, SubAssign};

pub struct BIT<T> {
    n: usize,
    bits: Vec<T>,
}
 
impl<T: Default + Copy + AddAssign + SubAssign> BIT<T> {
    pub fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        BIT {
            n: m,
            bits: vec![T::default(); m + 1],
        }
    }
 
    pub fn add(&mut self, a: usize, w: T) {
        let mut x = a as i32 + 1;
        while x <= self.n as i32 {
            self.bits[(x as usize)] += w;
            x += x&-x;
        }
    }
    
    pub fn sub(&mut self, a: usize, w: T) {
        let mut x = a as i32 + 1;
        while x <= self.n as i32 {
            self.bits[(x as usize)] -= w;
            x += x&-x;
        }
    }
 
    pub fn sum(&self, a: usize) -> T {
        let mut ret = T::default();
        let mut x = a as i32 + 1;
        while x > 0 {
            ret += self.bits[x as usize];
            x -= x&-x;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_test() {
        let mut bit = BIT::new(8);

        bit.add(1, 1);
        bit.add(3, 5);

        assert_eq!(0, bit.sum(0));
        assert_eq!(1, bit.sum(1));
        assert_eq!(1, bit.sum(2));
        assert_eq!(6, bit.sum(3));
        assert_eq!(6, bit.sum(4));
        assert_eq!(6, bit.sum(7));

        bit.sub(4, 2);
        assert_eq!(0, bit.sum(0));
        assert_eq!(1, bit.sum(1));
        assert_eq!(1, bit.sum(2));
        assert_eq!(6, bit.sum(3));
        assert_eq!(4, bit.sum(4));
        assert_eq!(4, bit.sum(7));
    }
}

