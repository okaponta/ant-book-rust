use proconio::input;

fn main() {
    input! {
        x:usize,
        k:usize,
        a:[usize;k],
    }
    let mut alice = vec![false; x + 1];
    for i in 1..=x {
        for j in 0..k {
            alice[i] |= a[j] <= i && !alice[i - a[j]];
        }
    }
    println!("{}", if alice[x] { "Alice" } else { "Bob" })
}
