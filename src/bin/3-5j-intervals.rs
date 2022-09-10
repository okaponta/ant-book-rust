use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:i64,
        abw:[(usize,usize,i64);n],
    }
    let mut x = vec![];
    for i in 0..n {
        x.push(abw[i].0);
        x.push(abw[i].1);
    }
    let map = compress(&x);
    let mut mcf = MinCostFlow::new(map.len() + 2);
    mcf.add_edges(0, 1, k, 0);
    mcf.add_edges(map.len(), map.len() + 1, k, 0);
    for i in 1..map.len() {
        mcf.add_edges(i, i + 1, 1 << 60, 0);
    }
    let mut ans = 0;
    for (a, b, w) in abw {
        // aからbにコスト-wの辺を張る
        mcf.add_edges(map[&b], map[&a], 1, w);
        mcf.add_edges(0, map[&b], 1, 0);
        mcf.add_edges(map[&a], map.len() + 1, 1, 0);
        ans -= w;
    }
    ans += mcf.min_cost_flow(0, map.len() + 1, k + n as i64);
    println!("{}", -ans);
}

fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i + 1);
    }
    result
}

// 最小費用流
pub struct MinCostFlow {
    n: usize,
    edge: Vec<Vec<(usize, i64, i64, usize)>>,
    h: Vec<i64>,
    dist: Vec<i64>,
    prevv: Vec<usize>,
    preve: Vec<usize>,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        let edge = vec![vec![]; n];
        let h = vec![0; n];
        let prevv = vec![0; n];
        let preve = vec![0; n];
        let dist = vec![1 << 60; n];
        Self {
            n,
            edge,
            h,
            dist,
            prevv,
            preve,
        }
    }

    pub fn add_edges(&mut self, from: usize, to: usize, cap: i64, cost: i64) {
        let idx = self.edge[from].len();
        let rev_idx = self.edge[to].len();
        self.edge[from].push((to, cap, cost, rev_idx));
        self.edge[to].push((from, 0, -cost, idx));
    }

    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i64) -> i64 {
        let mut res = 0;
        // hの初期化
        self.h = vec![0; self.n];
        let mut heap = std::collections::BinaryHeap::new();
        while f > 0 {
            // ダイクストラ法を用いてhを更新
            self.dist = vec![1 << 60; self.n];
            self.dist[s] = 0;
            heap.push((std::cmp::Reverse(0), s));
            while let Some((std::cmp::Reverse(d), v)) = heap.pop() {
                if self.dist[v] < d {
                    continue;
                }
                for i in 0..self.edge[v].len() {
                    let (to, cap, cost, _) = self.edge[v][i];
                    if cap > 0 && self.dist[to] > self.dist[v] + cost + self.h[v] - self.h[to] {
                        self.dist[to] = self.dist[v] + cost + self.h[v] - self.h[to];
                        self.prevv[to] = v;
                        self.preve[to] = i;
                        heap.push((std::cmp::Reverse(self.dist[to]), to));
                    }
                }
            }
            // これ以上流せない
            if self.dist[t] == 1 << 60 {
                return -1;
            }
            for v in 0..self.n {
                self.h[v] += self.dist[v];
            }

            // 最短経路に流す
            let mut d = f;
            let mut v = t;
            while v != s {
                // 流せるキャパシティを計算
                d = d.min(self.edge[self.prevv[v]][self.preve[v]].1);
                v = self.prevv[v];
            }
            f -= d;
            res += d * self.h[t];
            let mut v = t;
            while v != s {
                let rev = self.edge[self.prevv[v]][self.preve[v]].3;
                self.edge[self.prevv[v]][self.preve[v]].1 -= d;
                self.edge[v][rev].1 += d;
                v = self.prevv[v];
            }
        }
        res
    }
}
