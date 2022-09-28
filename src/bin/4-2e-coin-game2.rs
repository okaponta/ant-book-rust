use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
        x:[usize;n],
    }
    let maxx = *x.iter().max().unwrap();
    let mut grundy = vec![0; maxx + 1];
    for i in 1..=maxx {
        let mut set = HashSet::new();
        for j in 0..k {
            if a[j] <= i {
                set.insert(grundy[i - a[j]]);
            }
        }
        let mut g = 0;
        while set.contains(&g) {
            g += 1;
        }
        grundy[i] = g;
    }
    let mut xg = 0;
    for i in 0..n {
        xg ^= grundy[x[i]];
    }
    println!("{}", if xg != 0 { "Alice" } else { "Bob" })
}
