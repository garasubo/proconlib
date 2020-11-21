pub struct SegTree<T, F> {
    n: usize,
    node: Vec<T>,
    acc: F,
    default: T,
}

impl<T: Default + Clone, F> SegTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, acc: F) -> SegTree<T, F> {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        SegTree {
            n: m,
            node: vec![T::default(); m * 2],
            acc: acc,
            default: T::default(),
        }
    }
}
impl<T: Clone, F> SegTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    pub fn new_with_default(n: usize, acc: F, default: T) -> SegTree<T, F> {
        let mut m = 1;
        while m < n {
            m *= 2;
        }
        SegTree {
            n: m,
            node: vec![default.clone(); m * 2],
            acc: acc,
            default,
        }
    }

    pub fn add(&mut self, pos: usize, val: T) {
        let mut x = pos + self.n - 1;
        self.node[x] = (self.acc)(&self.node[x], &val);
        while x > 0 {
            x = (x - 1) / 2;
            self.node[x] = (self.acc)(&self.node[x], &val);
        }
    }

    fn sum_inner(&self, pos: usize, x: usize, y: usize, p: usize, q: usize) -> T {
        if x <= p && q <= y {
            return self.node[pos].clone();
        }
        if y <= p || q <= x {
            return self.default.clone();
        }

        let left = pos * 2 + 1;
        let mid = (p + q) / 2;
        let right = pos * 2 + 2;
        (self.acc)(
            &self.sum_inner(left, x, y, p, mid),
            &self.sum_inner(right, x, y, mid, q),
        )
    }

    pub fn sum(&self, x: usize, y: usize) -> T {
        self.sum_inner(0, x, y, 0, self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_test() {
        let mut tree = SegTree::new(8, |a: &i32, b: &i32| a + b);

        tree.add(1, 1);
        tree.add(3, 5);

        assert_eq!(0, tree.sum(0, 1));
        assert_eq!(1, tree.sum(0, 2));
        assert_eq!(1, tree.sum(0, 3));
        assert_eq!(6, tree.sum(0, 4));
        assert_eq!(5, tree.sum(2, 4));

        let mut tree = SegTree::new(8, |a: &i32, b: &i32| std::cmp::max(*a, *b));
        tree.add(1, 1);
        tree.add(3, 5);

        assert_eq!(0, tree.sum(0, 1));
        assert_eq!(1, tree.sum(0, 2));
        assert_eq!(1, tree.sum(0, 3));
        assert_eq!(5, tree.sum(0, 4));
        assert_eq!(5, tree.sum(2, 4));
    }
}
