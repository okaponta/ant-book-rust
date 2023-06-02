use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut st = s.clone();
    st.push('$');
    for i in (0..n).rev() {
        st.push(s[i]);
    }

    let sa = suffix_array(&st);
    let lcp = lcp(&st, &sa);
    let mut rank = vec![0; 2 * n + 2];
    for i in 0..=st.len() {
        rank[sa[i]] = i;
    }
    let mut rmq = SegmentTree::new(2 * n + 2, 1 << 60, |a, b| a.min(b));
    for i in 0..lcp.len() {
        rmq.update_tmp(i, lcp[i]);
    }
    rmq.update_all();

    let mut ans = 0;

    // iを中心とする奇数長の回文
    for i in 0..n {
        let j = n * 2 - i;
        let l = rmq.query(rank[i].min(rank[j])..1 + rank[i].max(rank[j]));
        if l != 0 {
            ans = ans.max(2 * l - 1);
        }
    }

    // 文字i-1と文字iを中心とする偶数長の回分
    for i in 1..n {
        let j = n * 2 - i + 1;
        let l = rmq.query(rank[i].min(rank[j])..1 + rank[i].max(rank[j]));
        ans = ans.max(2 * l);
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

pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Copy, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let m = size.next_power_of_two();
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f,
            initial_value,
        }
    }

    pub fn update(&mut self, mut k: usize, value: T) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    pub fn update_tmp(&mut self, k: usize, value: T) {
        self.seg[k + self.n - 1] = value;
    }

    pub fn update_all(&mut self) {
        for i in (0..self.n - 1).rev() {
            self.seg[i] = (self.f)(self.seg[2 * i + 1], self.seg[2 * i + 2]);
        }
    }

    // 半開区間なので注意
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> T {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            self.initial_value
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k]
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end);
            (self.f)(x, y)
        }
    }

    // lowerとupperの間でfを満たす最小の値
    // ng, ng, ng, (ok), ok, ok
    pub fn max_right<P>(&self, mut lower: usize, mut upper: usize, f: P) -> usize
    where
        P: Fn(T) -> bool,
    {
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if f(self.query(0..med)) {
                upper = med;
            } else {
                lower = med;
            }
        }
        upper
    }

    // lowerとupperの間でfを満たさない最大の値
    pub fn min_left<P>(&self, mut lower: usize, mut upper: usize, f: P) -> usize
    where
        P: Fn(T) -> bool,
    {
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if f(self.query(0..med)) {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}
