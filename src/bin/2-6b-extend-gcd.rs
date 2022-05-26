use proconio::input;

fn main() {
    input! {
        a:i64,b:i64
    }
    if gcd(a, b) != 1 {
        println!("-1");
        return;
    }
    let (mut x, mut y) = (0, 0);
    extend_euclid(a, b, &mut x, &mut y);
    println!("{} {} {} {}", x.max(0), y.max(0), -x.min(0), -y.min(0));
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn extend_euclid(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let d = extend_euclid(b, a % b, y, x);
    *y -= a / b * *x;
    d
}
