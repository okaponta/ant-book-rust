use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        l:i64,
        p:i64,
        a:[i64;n],
        b:[i64;n],
    }
    let mut ab = vec![];
    for i in 0..n {
        ab.push((a[i], b[i]));
    }
    ab.push((l, 0));
    ab.sort();
    let mut position = 0;
    let mut tank = p;
    let mut ans = 0;
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    for (ai, bi) in ab {
        let d = ai - position;
        while tank - d < 0 {
            if heap.is_empty() {
                println!("-1");
                return;
            }
            tank += heap.pop().unwrap();
            ans += 1;
        }
        tank -= d;
        position = ai;
        heap.push(bi);
    }
    println!("{}", ans);
}
