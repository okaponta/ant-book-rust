use proconio::input;

// 包除原理
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
    }
    let mut ans = 0;
    for i in 1usize..1 << m {
        let num = i.count_ones();
        let mut lcm = 1;
        for j in 0..m {
            if i >> j & 1 == 1 {
                lcm = lcm / gcd(lcm, a[j]) * a[j];
                if n < lcm {
                    break;
                }
            }
        }
        if num % 2 == 0 {
            ans -= n / lcm;
        } else {
            ans += n / lcm;
        }
    }
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
