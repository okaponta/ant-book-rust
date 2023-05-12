use std::cmp::Ordering;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        s:Chars,
    }
    println!("{}", suffix_array(&s).iter().join(" "));
}

fn suffix_array(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut sa = vec![0; n + 1];
    let mut rank = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    for i in 0..=n {
        sa[i] = i;
        rank[i] = if i < n { s[i] as i32 } else { -1 };
    }

    let mut k = 1;
    while k <= n {
        sa.sort_by(|&a, &b| compare_sa(a, b, k, n, &rank));
        tmp[sa[0]] = 0;
        for i in 1..=n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if compare_sa(sa[i - 1], sa[i], k, n, &rank) == Ordering::Less {
                    1
                } else {
                    0
                };
        }
        for i in 0..=n {
            rank[i] = tmp[i];
        }
        k *= 2;
    }
    sa
}

fn compare_sa(i: usize, j: usize, k: usize, n: usize, rank: &Vec<i32>) -> Ordering {
    if rank[i] != rank[j] {
        return rank[i].cmp(&rank[j]);
    }
    let ri = if i + k <= n { rank[i + k] } else { -1 };
    let rj = if j + k <= n { rank[j + k] } else { -1 };
    ri.cmp(&rj)
}
