use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let sl = s.len();
    let mut st = s.clone();
    st.push('0');
    st.extend(t);

    let sa = suffix_array(&st);
    let lcp = lcp(&st, &sa);

    let mut ans = 0;
    for i in 0..st.len() {
        if (sa[i] < sl) != (sa[i + 1] < sl) {
            ans = ans.max(lcp[i]);
        }
    }
    println!("{}", ans);
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

    fn compare_sa(i: usize, j: usize, k: usize, n: usize, rank: &Vec<i32>) -> Ordering {
        if rank[i] != rank[j] {
            return rank[i].cmp(&rank[j]);
        }
        let ri = if i + k <= n { rank[i + k] } else { -1 };
        let rj = if j + k <= n { rank[j + k] } else { -1 };
        ri.cmp(&rj)
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

fn lcp(s: &Vec<char>, sa: &Vec<usize>) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0; n + 1];
    for i in 0..=n {
        rank[sa[i]] = i;
    }

    let mut h = 0;
    let mut lcp = vec![0; n + 1];
    for i in 0..n {
        let j = sa[rank[i] - 1];
        if 0 < h {
            h -= 1;
        }
        while j + h < n && i + h < n {
            if s[j + h] != s[i + h] {
                break;
            }
            h += 1;
        }
        lcp[rank[i] - 1] = h;
    }
    lcp
}
