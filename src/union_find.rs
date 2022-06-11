#[derive(Clone, Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut par = Vec::with_capacity(n);
        for i in 0..n {
            par.push(i);
        }
        UnionFind {
            parent: par,
            size: vec![1; n],
        }
    }

    pub fn union(&mut self, l: usize, r: usize) {
        let lp = self.get_par(l);
        let rp = self.get_par(r);

        if lp != rp {
            self.parent[lp] = rp;
            self.size[rp] += self.size[lp];
        }
    }

    pub fn get_par(&mut self, tar: usize) -> usize {
        let mut par = self.parent[tar];
        if self.parent[par] != par {
            par = self.get_par(par);
            self.parent[tar] = par;
        }
        par
    }

    pub fn get_par_size(&mut self, tar: usize) -> usize {
        let par = self.get_par(tar);
        self.size[par]
    }

    pub fn is_same(&mut self, l: usize, r: usize) -> bool {
        let lp = self.get_par(l);
        let rp = self.get_par(r);

        lp == rp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_test() {
        let mut union_find = UnionFind::new(10);

        union_find.union(3, 0);
        union_find.union(1, 2);
        union_find.union(4, 6);
        union_find.union(9, 5);

        union_find.union(1, 3);
        union_find.union(2, 7);
        union_find.union(8, 9);

        assert!(union_find.is_same(0, 3));
        assert!(union_find.is_same(0, 7));
        assert!(union_find.is_same(0, 7));
        assert!(union_find.is_same(5, 8));
        assert!(!union_find.is_same(0, 4));
        assert!(!union_find.is_same(6, 8));
        assert!(!union_find.is_same(1, 9));
        assert_eq!(5, union_find.get_par_size(0));
        assert_eq!(5, union_find.get_par_size(1));
        assert_eq!(2, union_find.get_par_size(4));
        assert_eq!(3, union_find.get_par_size(5));
    }
}
