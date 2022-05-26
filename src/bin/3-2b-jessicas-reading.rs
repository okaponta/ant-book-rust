use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        p:usize,
        a:[usize;p],
    }
    let size = a.iter().collect::<HashSet<_>>().len();
    let mut ans = 1 << 60;
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut map = HashMap::new();
    let mut types = 0;
    loop {
        while types < size && right < p {
            if !map.contains_key(&a[right]) {
                types += 1;
            }
            *map.entry(a[right]).or_insert(0) += 1;
            right += 1;
        }
        if types < size {
            // しゃくとり終了
            break;
        }
        ans = ans.min(right - left);
        while types == size {
            *map.entry(a[left]).or_insert(0) -= 1;
            if map.get(&a[left]).unwrap() == &0 {
                map.remove(&a[left]);
                types -= 1;
            }
            left += 1;
        }
    }
    println!("{}", ans);
}
