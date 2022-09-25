use num_integer::Roots;
use proconio::input;

const MOD: i64 = 10009;

fn main() {
    input! {
        n:i64,
    }
    let mut ans = 0;
    for (divisor, coef) in moebius(n) {
        ans += coef * pow(26, n / divisor, MOD);
        ans = (ans + MOD) % MOD;
    }
    println!("{}", ans);
}

fn moebius(n: i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    // 平方数で割り切れるものは除外する
    let primes = factorize(n).iter().map(|(f, _)| *f).collect::<Vec<_>>();
    let m = primes.len();
    for i in 0..1 << m {
        let mut mu = 1;
        let mut d = 1;
        for j in 0..m {
            if i >> j & 1 == 1 {
                mu *= -1;
                d *= primes[j];
            }
        }
        res.push((d, mu));
    }
    res
}

fn factorize(mut n: i64) -> Vec<(i64, i64)> {
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

fn pow(mut x: i64, mut n: i64, modulo: i64) -> i64 {
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
