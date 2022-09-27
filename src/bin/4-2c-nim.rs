use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut x = 0;
    for a in a {
        x ^= a;
    }
    println!("{}", if x != 0 { "Alice" } else { "Bob" })
}
