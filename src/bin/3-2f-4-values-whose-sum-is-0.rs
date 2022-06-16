use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
        b:[i64;n],
        c:[i64;n],
        d:[i64;n],
    }
    let mut ab = HashMap::new();
    let mut cd = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            *ab.entry(a[i] + b[j]).or_insert(0) += 1;
            *cd.entry(c[i] + d[j]).or_insert(0) += 1;
        }
    }
    let mut ans = 0;
    for (k, v) in ab {
        ans += cd.get(&-k).unwrap_or(&0) * v;
    }
    println!("{}", ans);
}
