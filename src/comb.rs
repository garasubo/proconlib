
const MOD: i64 = 1_000_000_007;

fn extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut d = a;
    if b != 0 {
        d = extgcd(b, a % b, y, x);
        *y -= (a / b) * (*x);
    } else {
        *x = 1; *y = 0;
    }
    d
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let mut x = 0;
    let mut y = 0;
    extgcd(a, m, &mut x, &mut y);
    (m + x % m) % m
}

pub struct Comb {
    memo: Vec<i64>,
}

impl Comb {
    pub fn new(n: usize) -> Self {
        let mut memo = vec![0; n];
        memo[0] = 1;
        memo[1] = 1;
        for i in 2..memo.len() {
            memo[i] = (memo[i-1] * (i as i64)) % MOD;
        }
        Comb {
            memo,
        }
    }

    pub fn mod_comb(&self, n: i64, k: i64) -> i64 {
        if n < 0 || k < 0 || n < k {
            return 0;
        }
        let a1 = self.memo[n as usize];
        let a2 = self.memo[k as usize];
        let a3 = self.memo[(n-k) as usize];
        a1 * mod_inverse(a2, MOD) % MOD * mod_inverse(a3, MOD) % MOD
    }
    
    pub fn mod_perm(&self, n: i64, k: i64) -> i64 {
        if n < 0 || k < 0 || n < k {
            return 0;
        }
        let a1 = self.memo[n as usize];
        let a3 = self.memo[(n-k) as usize];
        a1 * mod_inverse(a3, MOD) % MOD
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_test() {
        let comb = Comb::new(11);

        assert_eq!(45, comb.mod_comb(10, 2));
        assert_eq!(120, comb.mod_comb(10, 3));
        assert_eq!(210, comb.mod_comb(10, 4));
        
        assert_eq!(90, comb.mod_perm(10, 2));
        assert_eq!(720, comb.mod_perm(10, 3));
        assert_eq!(5040, comb.mod_perm(10, 4));
    }
}