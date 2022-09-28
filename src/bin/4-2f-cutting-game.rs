use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        w:usize,
        h:usize,
    }
    let max = 200;
    let mut g = vec![vec![-1; max + 1]; max + 1];
    println!(
        "{}",
        if grundy(w, h, &mut g) != 0 {
            "WIN"
        } else {
            "LOSE"
        }
    );
}

fn grundy(w: usize, h: usize, g: &mut Vec<Vec<i64>>) -> i64 {
    if g[w][h] != 1 {
        return g[w][h];
    }
    let mut s = HashSet::new();
    for i in 2..w - 1 {
        s.insert(grundy(i, h, g) ^ grundy(w - i, h, g));
    }
    for i in 2..h - 1 {
        s.insert(grundy(w, i, g) ^ grundy(w, h - i, g));
    }
    let mut res = 0;
    while s.contains(&res) {
        res += 1;
    }
    res
}
