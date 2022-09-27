use proconio::input;

fn main() {
    input! {
        n:usize,
        mut p:[usize;n],
    }
    if n % 2 == 1 {
        p.insert(0, 0);
    }
    let mut x = 0;
    for i in (0..n).step_by(2) {
        x ^= p[i + 1] - p[i] - 1;
    }
    println!("{}", if x != 0 { "Georgia" } else { "Bob" })
}
