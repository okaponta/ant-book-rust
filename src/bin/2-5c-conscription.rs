use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        r:usize,
        xyd:[(usize,usize,i64);r],
    }
    let edges = xyd
        .iter()
        .map(|&(x, y, d)| (x, n + y, -d))
        .collect::<Vec<_>>();
    println!("{}", 10000 * (n + m) as i64 + kruskal(n + m, edges));
}

fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> i64 {
    edges.sort_by_key(|e| e.2);
    let mut uf = UnionFind::new(n);
    let mut res = 0;
    for (u, v, cost) in edges {
        if uf.union(u, v) {
            res += cost;
        }
    }
    res
}
