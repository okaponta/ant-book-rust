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
    let mut edges = vec![vec![]; 2 * n];
    let mut rev_edges = vec![vec![]; 2 * n];
    for i in 0..n {
        for j in 0..i {
            if std[i].0.max(std[j].0) < (std[i].0 + std[i].2).min(std[j].0 + std[j].2) {
                // 最初 and 最初で時間がかぶる
                // not x[i] or not x[j]
                // x[i] -> not x[j] and x[j] -> not x[i]
                edges[i].push(n + j);
                rev_edges[n + j].push(i);
                edges[j].push(n + i);
                rev_edges[n + i].push(j);
            }
            if std[i].0.max(std[j].1 - std[j].2) < (std[i].0 + std[i].1).min(std[j].1) {
                // 最初 and 最後で時間がかぶる
                // not x[i] or x[j]
                // x[i] -> x[j] and not x[j] -> not x[i]
                edges[i].push(j);
                rev_edges[j].push(i);
                edges[n + j].push(n + i);
                rev_edges[n + i].push(n + j);
            }
            if (std[i].1 - std[i].2).max(std[j].0) < std[i].1.min(std[j].0 + std[j].2) {
                edges[n + i].push(n + j);
                rev_edges[n + j].push(n + i);
                edges[j].push(i);
                rev_edges[i].push(j);
            }
            if (std[i].1 - std[i].2).max(std[j].1 - std[j].2) < std[i].0.min(std[j].0) {
                edges[n + i].push(j);
                rev_edges[j].push(n + i);
                edges[n + j].push(i);
                rev_edges[i].push(n + j);
            }
        }
    }
    scc.execute(edges, rev_edges);

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
    sizes: Vec<usize>,
    new_num: Vec<usize>,
    new_edges: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        let sizes = vec![];
        let new_num = vec![0; n];
        let new_edges = vec![];
        Self {
            n,
            sizes,
            new_num,
            new_edges,
        }
    }

    // edges/rev_edges もとの辺
    pub fn execute(&mut self, edges: Vec<Vec<usize>>, rev_edges: Vec<Vec<usize>>) {
        let n = self.n;
        let mut used = vec![false; n];
        // num[i] -> i番目の番号がどの頂点か(一度目のdfsの結果を記録)
        let mut num = vec![0; n];
        let mut count = 0;
        for i in 0..n {
            if !used[i] {
                count = self.dfs(i, count, &mut used, &mut num, &edges);
            }
        }
        // 初期化して二度目のdfsで使い回し
        used = vec![false; n];
        let mut count = 0;
        for i in (0..n).rev() {
            let target = num[i];
            if !used[target] {
                let size = self.rev_dfs(target, count, 0, &mut used, &rev_edges);
                self.sizes.push(size);
                count += 1;
            }
        }
        let mut new_edges = vec![std::collections::BTreeSet::new(); self.sizes.len()];
        for i in 0..n {
            for &edge in &edges[i] {
                if self.new_num[i] != self.new_num[edge] {
                    new_edges[self.new_num[i]].insert(self.new_num[edge]);
                }
            }
        }
        self.new_edges = new_edges
            .iter()
            .map(|s| s.iter().map(|i| *i).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        //return (sizes, new_num, v);
    }

    fn dfs(
        &mut self,
        cur: usize,
        mut count: usize,
        used: &mut Vec<bool>,
        num: &mut Vec<usize>,
        edges: &Vec<Vec<usize>>,
    ) -> usize {
        used[cur] = true;
        for &next in edges[cur].iter() {
            if !used[next] {
                count = self.dfs(next, count, used, num, edges);
            }
        }
        num[count] = cur;
        count + 1
    }

    fn rev_dfs(
        &mut self,
        cur: usize,
        count: usize,
        mut size: usize,
        used: &mut Vec<bool>,
        rev_edges: &Vec<Vec<usize>>,
    ) -> usize {
        used[cur] = true;
        for &next in rev_edges[cur].iter() {
            if !used[next] {
                size = self.rev_dfs(next, count, size, used, rev_edges);
            }
        }
        self.new_num[cur] = count;
        size + 1
    }
}
