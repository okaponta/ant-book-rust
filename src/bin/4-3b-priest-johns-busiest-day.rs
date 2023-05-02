use proconio::input;

fn main() {
    input! {
        n:usize,
        std:[(usize,usize,usize);n],
    }
    // 2-SAT
    // a or b
    // not a -> b and not b -> a
    // (どっちかがfalseならもう片方がtrueじゃないとa or bがtrueにならない)
    // 今回の問題は
    // xi...iを最初に行う時、true
    // もしxiとxjを両方最初に行った時に時間がかぶるなら、(not xi) or (not xj)
    // (どちらかは最後に行う)
    let mut scc = SCC::new(2 * n);
    for i in 0..n {
        for j in 0..i {
            if std[i].0.max(std[j].0) < (std[i].0 + std[i].2).min(std[j].0 + std[j].2) {
                // 最初 and 最初で時間がかぶる
                // not x[i] or not x[j]
                // x[i] -> not x[j] and x[j] -> not x[i]
                scc.add_edges(i, n + j);
                scc.add_edges(j, n + i);
            }
            if std[i].0.max(std[j].1 - std[j].2) < (std[i].0 + std[i].1).min(std[j].1) {
                // 最初 and 最後で時間がかぶる
                // not x[i] or x[j]
                // x[i] -> x[j] and not x[j] -> not x[i]
                scc.add_edges(i, j);
                scc.add_edges(n + j, n + i);
            }
            if (std[i].1 - std[i].2).max(std[j].0) < std[i].1.min(std[j].0 + std[j].2) {
                scc.add_edges(n + i, n + j);
                scc.add_edges(j, i);
            }
            if (std[i].1 - std[i].2).max(std[j].1 - std[j].2) < std[i].0.min(std[j].0) {
                scc.add_edges(n + i, j);
                scc.add_edges(n + j, i);
            }
        }
    }
    scc.execute();

    for i in 0..n {
        if scc.new_num[i] == scc.new_num[n + i] {
            println!("NO");
            return;
        }
    }

    println!("YES");
    for i in 0..n {
        if scc.new_num[n + i] < scc.new_num[i] {
            println!("{} {}", std[i].0, std[i].0 + std[i].2);
        } else {
            println!("{} {}", std[i].1 - std[i].2, std[i].1);
        }
    }
}

// SCC(強連結成分分解)
// n もとの頂点数
// sizes 強連結成分をまとめたときのサイズ
// new_num もとの頂点->まとめたあとの頂点のマッピング
// new_edges まとめたあとの辺(トポロジカルソート済)
pub struct SCC {
    n: usize,
    g: Vec<Vec<usize>>,
    rev_g: Vec<Vec<usize>>,
    sizes: Vec<usize>,
    new_num: Vec<usize>,
    new_edges: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        let g = vec![vec![]; n];
        let rev_g = vec![vec![]; n];
        let sizes = vec![];
        let new_num = vec![0; n];
        let new_edges = vec![];
        Self {
            n,
            g,
            rev_g,
            sizes,
            new_num,
            new_edges,
        }
    }

    pub fn add_edges(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.rev_g[v].push(u);
    }

    // edges/rev_edges もとの辺
    pub fn execute(&mut self) {
        let n = self.n;
        let mut used = vec![false; n];
        // num[i] -> i番目の番号がどの頂点か(一度目のdfsの結果を記録)
        let mut num = vec![0; n];
        let mut count = 0;

        fn dfs(
            cur: usize,
            mut count: usize,
            used: &mut Vec<bool>,
            num: &mut Vec<usize>,
            edges: &Vec<Vec<usize>>,
        ) -> usize {
            used[cur] = true;
            for &next in edges[cur].iter() {
                if !used[next] {
                    count = dfs(next, count, used, num, edges);
                }
            }
            num[count] = cur;
            count + 1
        }

        for i in 0..n {
            if !used[i] {
                count = dfs(i, count, &mut used, &mut num, &self.g);
            }
        }
        // 初期化して二度目のdfsで使い回し
        used = vec![false; n];
        let mut count = 0;

        fn rev_dfs(
            cur: usize,
            count: usize,
            mut size: usize,
            new_num: &mut Vec<usize>,
            used: &mut Vec<bool>,
            rev_edges: &Vec<Vec<usize>>,
        ) -> usize {
            used[cur] = true;
            for &next in rev_edges[cur].iter() {
                if !used[next] {
                    size = rev_dfs(next, count, size, new_num, used, rev_edges);
                }
            }
            new_num[cur] = count;
            size + 1
        }

        for i in (0..n).rev() {
            let target = num[i];
            if !used[target] {
                let size = rev_dfs(target, count, 0, &mut self.new_num, &mut used, &self.rev_g);
                self.sizes.push(size);
                count += 1;
            }
        }
        let mut new_edges = vec![std::collections::BTreeSet::new(); self.sizes.len()];
        for i in 0..n {
            for &edge in &self.g[i] {
                if self.new_num[i] != self.new_num[edge] {
                    new_edges[self.new_num[i]].insert(self.new_num[edge]);
                }
            }
        }
        self.new_edges = new_edges
            .iter()
            .map(|s| s.iter().map(|i| *i).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }
}
