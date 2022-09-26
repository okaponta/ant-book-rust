use num_integer::Roots;
use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let primes = factorize(n);
    let divs = divisor(n);
    let mut ans = 0;
    for &div in &divs {
        let mut euler = div;
        for (p, _) in &primes {
            if div % p == 0 {
                euler = euler / p * (p - 1);
            }
        }
        ans += euler * pow(m, n / div, MOD);
        ans %= MOD;
    }
    println!("{}", ans * pow(n, MOD - 2, MOD) % MOD);
}

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

fn divisor(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut upper = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                upper.push(n / i);
            }
        }
    }
    upper.reverse();
    res.append(&mut upper);
    res
}

fn pow(mut x: usize, mut n: usize, modulo: usize) -> usize {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
