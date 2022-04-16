use proconio::{input, marker::Usize1};

// Ford-Fulkerson
fn main() {
    input! {
        n:usize,
        m:usize,
        stc:[(Usize1,Usize1,i64);m],
    }
    let mut ff = FordFulkerson::new(n);
    for (s, t, c) in stc {
        ff.add_edges(s, t, c);
    }
    println!("{}", ff.max_flow());
}

struct FordFulkerson {
    n: usize,
    edge: Vec<Vec<(usize, i64, usize)>>,
    flow: i64,
}

impl FordFulkerson {
    pub fn new(n: usize) -> Self {
        let flow = 0;
        let edge = vec![vec![]; n];
        Self { n, edge, flow }
    }

    pub fn add_edges(&mut self, from: usize, to: usize, cap: i64) {
        let idx = self.edge[from].len();
        let rev_idx = self.edge[to].len();
        self.edge[from].push((to, cap, rev_idx));
        self.edge[to].push((from, 0, idx));
    }

    pub fn max_flow(&mut self) -> i64 {
        loop {
            if self.next_flow() == 0 {
                return self.flow;
            }
        }
    }

    pub fn next_flow(&mut self) -> i64 {
        let f = self.dfs(0, self.n - 1, 1 << 60, &mut vec![false; self.n]);
        self.flow += f;
        f
    }

    pub fn dfs(&mut self, from: usize, to: usize, flow: i64, used: &mut Vec<bool>) -> i64 {
        if from == to {
            return flow;
        }
        used[from] = true;
        for i in 0..self.edge[from].len() {
            let (next, cap, rev) = self.edge[from][i];
            if !used[next] && cap > 0 {
                let d = self.dfs(next, to, flow.min(cap), used);
                if d > 0 {
                    self.edge[from][i].1 -= d;
                    self.edge[next][rev].1 += d;
                    return d;
                }
            }
        }
        0
    }
}
