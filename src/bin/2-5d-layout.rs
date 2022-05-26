use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n:usize,
        ml:usize,
        md:usize,
        mut dl:[(Usize1,Usize1,i64);ml],
        mut dd:[(Usize1,Usize1,i64);md],
    }
    let mut edges = vec![];
    for i in 1..n {
        edges.push((i, i - 1, 0));
    }
    for (a, b, d) in dl {
        edges.push((a, b, d));
    }
    for (a, b, d) in dd {
        edges.push((b, a, -d));
    }
    let bf = BellmanFord::new(n, edges, 0);
    if bf.has_neg_loop {
        println!("-1");
        return;
    }
    if bf.distance[n - 1] == INF {
        println!("-2");
        return;
    }
    println!("{}", bf.distance[n - 1]);
}

struct BellmanFord {
    distance: Vec<i64>,
    has_neg_loop: bool,
}

impl BellmanFord {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        distance[init] = 0;
        let mut has_neg_loop = false;

        for i in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    distance[to] = distance[from] + cost;
                    if i == n - 1 {
                        has_neg_loop = true;
                        break;
                    }
                }
            }
        }
        Self {
            distance,
            has_neg_loop,
        }
    }
}
