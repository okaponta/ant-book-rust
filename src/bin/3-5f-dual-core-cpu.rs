use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(i64,i64);n],
        abw:[(usize,usize,i64);m],
    }
    // 0 source
    // 1-n モジュール
    // n+1 sink
    let mut dinic = Dinic::new(n + 2);
    for i in 0..n {
        dinic.add_edges(0, i + 1, ab[i].0);
        dinic.add_edges(i + 1, n + 1, ab[i].1);
    }
    for (a, b, w) in abw {
        dinic.add_edges(a, b, w);
        dinic.add_edges(b, a, w);
    }
    println!("{}", dinic.max_flow());
}

pub struct Dinic {
    n: usize,
    edge: Vec<Vec<(usize, i64, usize)>>,
    level: Vec<i64>,
    iter: Vec<usize>,
    flow: i64,
}

impl Dinic {
    pub fn new(n: usize) -> Self {
        let flow = 0;
        let edge = vec![vec![]; n];
        let level = vec![];
        let iter = vec![];
        Self {
            n,
            edge,
            level,
            iter,
            flow,
        }
    }

    pub fn add_edges(&mut self, from: usize, to: usize, cap: i64) {
        let idx = self.edge[from].len();
        let rev_idx = self.edge[to].len();
        self.edge[from].push((to, cap, rev_idx));
        self.edge[to].push((from, 0, idx));
    }

    pub fn max_flow(&mut self) -> i64 {
        loop {
            self.bfs();
            if self.level[self.n - 1] < 0 {
                return self.flow;
            }
            self.iter = vec![0; self.n];
            loop {
                if self.next_flow() == 0 {
                    break;
                }
            }
        }
    }

    pub fn bfs(&mut self) {
        self.level = vec![-1; self.n];
        let mut q = std::collections::VecDeque::new();
        self.level[0] = 0;
        q.push_back(0);
        while let Some(v) = q.pop_front() {
            for i in 0..self.edge[v].len() {
                let (next, cap, _) = self.edge[v][i];
                if cap > 0 && self.level[next] < 0 {
                    self.level[next] = self.level[v] + 1;
                    q.push_back(next);
                }
            }
        }
    }

    pub fn next_flow(&mut self) -> i64 {
        let f = self.dfs(0, self.n - 1, 1 << 60);
        self.flow += f;
        f
    }

    pub fn dfs(&mut self, from: usize, to: usize, flow: i64) -> i64 {
        if from == to {
            return flow;
        }
        while self.iter[from] < self.edge[from].len() {
            let i = self.iter[from];
            let (next, cap, rev) = self.edge[from][i];
            if cap > 0 && self.level[from] < self.level[next] {
                let d = self.dfs(next, to, flow.min(cap));
                if d > 0 {
                    self.edge[from][i].1 -= d;
                    self.edge[next][rev].1 += d;
                    return d;
                }
            }
            self.iter[from] += 1;
        }
        0
    }
}
