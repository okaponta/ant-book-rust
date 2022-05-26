use num_integer::Roots;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    for ai in a {
        println!("{}", if judge_carmichael(ai) { "Yes" } else { "No" });
    }
}

fn judge_carmichael(n: i64) -> bool {
    let factors = factorize(n);
    if factors.len() < 3 {
        // 少なくとも3個以上の異なる素数の積
        return false;
    }
    for (p, _) in factors {
        if (n - 1) % (p - 1) != 0 {
            return false;
        }
    }
    true
}

// 素因数分解する。(素因数,累乗)の形式で返却する
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
