use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        s:Usize1,
        abw:[(Usize1,Usize1,usize);n-1],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[abw[i].0].push((abw[i].1, abw[i].2, i));
        g[abw[i].1].push((abw[i].0, abw[i].2, i));
    }
    let mut vs = vec![0; 2 * n];
    let mut dist = vec![0; 2 * n];
    let mut depth = vec![0; 2 * n];
    let mut id = vec![(0, 0); n];
    let mut query = vec![(0, 0); n - 1];
    dfs_vs_cost(
        0, 0, &g, &mut 0, 0, &mut vs, &mut dist, &mut depth, &mut id, &mut query,
    );

    let mut seg = SegmentTree::new(2 * n, (1 << 60, 0), |a, b| if a.0 < b.0 { a } else { b });
    for i in 0..2 * n {
        seg.update_tmp(i, (depth[i], i));
    }
    seg.update_all();

    let mut fw = FenwickTree::new(2 * n);
    for i in 0..2 * n {
        fw.add(i + 1, dist[i] as i64);
    }

    let mut cur = s;

    for _ in 0..q {
        input! {q: char}
        if q == 'A' {
            input! {x: Usize1}
            let u = id[cur].0.min(id[x].0);
            let v = id[cur].0.max(id[x].0) + 1;
            let (_, i) = seg.query(u..v);
            let ans = fw.sum(id[cur].0 + 1) + fw.sum(id[x].0 + 1) - 2 * fw.sum(id[i].0 + 1);
            println!("{}", ans);
            cur = x;
        } else {
            input! {x:Usize1, t:i64}
            fw.update(query[x].0 + 1, t);
            fw.update(query[x].1 + 1, -t);
        }
    }
}

fn dfs_vs_cost(
    prev: usize,
    cur: usize,
    edges: &Vec<Vec<(usize, usize, usize)>>,
    k: &mut usize,
    d: usize,
    vs: &mut Vec<usize>,
    dist: &mut Vec<i64>,
    depth: &mut Vec<usize>,
    id: &mut Vec<(usize, usize)>,
    query: &mut Vec<(usize, usize)>,
) {
    id[cur].0 = *k;
    vs[*k] = cur;
    depth[*k] = d;
    *k += 1;
    for &(next, cost, index) in &edges[cur] {
        if next == prev {
            continue;
        }
        dist[*k] = cost as i64;
        query[index].0 = *k;
        dfs_vs_cost(cur, next, edges, k, d + 1, vs, dist, depth, id, query);
        id[cur].1 = *k;
        vs[*k] = cur;
        depth[*k] = d;
        dist[*k] = -(cost as i64);
        query[index].1 = *k;
        *k += 1;
    }
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

// フェニック木。以下2つができる。1-indexedなので注意
// 1. ai に v を加算する
// 2. a1+a2+...+aiを求める
pub struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    pub fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    pub fn update(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let cur = self.range(i, i);
        self.add(i, v - cur);
    }

    // a1+a2+...aiを計算する
    pub fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }

    // ai+...+ajを計算する
    pub fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }

    // 和がs以下の位置を返却
    pub fn lower(&self, s: i64) -> usize {
        let mut lower = 0;
        let mut upper = self.len;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if self.sum(med) <= s {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}
