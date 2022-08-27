use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

// 最小費用流
fn main() {
    input! {
        n:usize,
        m:usize,
        stcd:[(Usize1,Usize1,i64,i64);m],
        f:i64,
    }
    let mut mcf = MinCostFlow::new(n);
    for (s, t, c, d) in stcd {
        mcf.add_edges(s, t, c, d);
    }
    println!("{}", mcf.min_cost_flow(0, n - 1, f));
}

struct MinCostFlow {
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
        let mut heap = BinaryHeap::new();
        while f > 0 {
            // ダイクストラ法を用いてhを更新
            self.dist = vec![1 << 60; self.n];
            self.dist[s] = 0;
            heap.push((Reverse(0), s));
            while let Some((Reverse(d), v)) = heap.pop() {
                if self.dist[v] < d {
                    continue;
                }
                for i in 0..self.edge[v].len() {
                    let (to, cap, cost, _) = self.edge[v][i];
                    if cap > 0 && self.dist[to] > self.dist[v] + cost + self.h[v] - self.h[to] {
                        self.dist[to] = self.dist[v] + cost + self.h[v] - self.h[to];
                        self.prevv[to] = v;
                        self.preve[to] = i;
                        heap.push((Reverse(self.dist[to]), to));
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
