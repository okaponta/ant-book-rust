use proconio::input;

fn main() {
    input! {
        p1:(i64,i64),
        p2:(i64,i64),
    }
    let dx = (p1.0 - p2.0).abs();
    let dy = (p1.1 - p2.1).abs();
    let ans;
    if dx == 0 {
        ans = dy;
    } else if dy == 0 {
        ans = dx;
    } else {
        ans = gcd(dx, dy);
    }
    println!("{}", if ans == 0 { 0 } else { ans - 1 });
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
