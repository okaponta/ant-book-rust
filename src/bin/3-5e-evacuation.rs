use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x:usize,
        y:usize,
        c:[Chars;x],
    }
    let mut dist_map = HashMap::new();
    let mut door = vec![];
    let mut person = vec![];
    for i in 0..x {
        for j in 0..y {
            if c[i][j] == 'D' {
                door.push((i, j));
                dist_map.insert((i, j), bfs(x, y, (i, j), &c, '.'));
            } else if c[i][j] == '.' {
                person.push((i, j));
            }
        }
    }
    // 0 sink
    // 1-d 時刻1に対応するドア
    // d+1-2d 時刻2に対応するドア
    // x*y tの最大値(ここ工夫できそう、少なくとも.の数にはなるはず)
    // x*y*d-x*y*d+p 人
    // 各時刻と人とのマッチングを求める
    let mut bi = Bipartite::new(x * y * door.len(), person.len());
    for i in 0..door.len() {
        for j in 0..person.len() {
            if dist_map[&door[i]][person[j].0][person[j].1] != 1 << 60 {
                for k in dist_map[&door[i]][person[j].0][person[j].1]..=x * y {
                    bi.join_edge((k - 1) * door.len() + i, j);
                }
            }
        }
    }
    if person.len() == 0 {
        println!("0");
        return;
    }
    let mut num = 0;
    for v in 1..=x * y * door.len() {
        let mut used = vec![false; x * y * door.len() + person.len() + 2];
        // 最大マッチングの必要な部分の処理だけを実行
        if bi.dfs(v, &mut used) {
            num += 1;
            if num == person.len() {
                // 1-dなら時刻1
                // d+1-2dなら時刻2
                println!("{}", (v - 1) / door.len() + 1);
                return;
            }
        }
    }
    println!("impossible")
}

fn bfs(
    h: usize,
    w: usize,
    init: (usize, usize),
    grid: &Vec<Vec<char>>,
    ok: char,
) -> Vec<Vec<usize>> {
    let mut res = vec![vec![1 << 60; w]; h];
    let mut q = VecDeque::new();
    res[init.0][init.1] = 0usize;
    q.push_back((init.0, init.1));
    while let Some((x, y)) = q.pop_front() {
        let d = res[x][y];
        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if h <= nx || w <= ny {
                continue;
            }
            if grid[nx][ny] == ok && d < res[nx][ny] {
                q.push_back((nx, ny));
                res[nx][ny] = d + 1;
            }
        }
    }
    res
}

#[allow(dead_code)]
struct Bipartite {
    n: usize,
    k: usize,
    edge: Vec<Vec<usize>>,
    matching: Vec<i64>,
}

// 0...source/n+k+1...sink
#[allow(dead_code)]
impl Bipartite {
    pub fn new(n: usize, k: usize) -> Self {
        let mut edge = vec![vec![]; n + k + 2];
        // source
        for i in 0..n {
            edge[0].push(i + 1);
            edge[i + 1].push(0);
        }
        // sink
        for i in 0..k {
            edge[n + k + 1].push(n + i + 1);
            edge[n + i + 1].push(n + k + 1);
        }
        let matching = vec![-1; n + k + 2];
        Self {
            n,
            k,
            edge,
            matching,
        }
    }

    // 一方の集合のa個目と一方の集合のb個目をつなぐ(0-index)
    pub fn join_edge(&mut self, a: usize, b: usize) {
        let a = a + 1;
        let b = self.n + b + 1;
        self.edge[a].push(b);
        self.edge[b].push(a);
    }

    pub fn max_matching(&mut self) -> i64 {
        let mut res = 0;
        for i in 0..self.n + self.k + 2 {
            if self.matching[i] < 0 {
                if self.dfs(i, &mut vec![false; self.n + self.k + 2]) {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn dfs(&mut self, cur: usize, used: &mut Vec<bool>) -> bool {
        used[cur] = true;
        for v in 0..self.edge[cur].len() {
            let u = self.edge[cur][v];
            let w = self.matching[v];
            if w < 0 || !used[w as usize] && self.dfs(w as usize, used) {
                self.matching[v] = u as i64;
                self.matching[u] = v as i64;
                return true;
            }
        }
        false
    }
}
