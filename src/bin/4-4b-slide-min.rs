use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    // インデックスを保持。
    // i1 < i2 かつ a[i1] < a[i2]を満たすように。
    // 先頭が対象インデックスなら削除する
    let mut q = VecDeque::new();
    let mut ans = vec![];
    for i in 0..n {
        while let Some(b) = q.pop_back() {
            if a[b] < a[i] {
                q.push_back(b);
                break;
            }
        }
        q.push_back(i);
        if k <= i + 1 {
            let f = q.pop_front().unwrap();
            ans.push(a[f]);
            if f != i + 1 - k {
                q.push_front(f);
            }
        }
    }
    println!("{}", ans.into_iter().join(" "));
}
