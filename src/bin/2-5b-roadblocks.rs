use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        r:usize,
        stc:[(Usize1,Usize1,usize);r],
    }
    let mut edge = vec![vec![]; n];
    for (s, t, c) in stc {
        edge[s].push((t, c));
        edge[t].push((s, c));
    }
    let d = Dijkstra::new(n, edge, 0);
    println!("{}", d.distance2[n - 1]);
}

const INF: usize = 1 << 60;

// 計算量は(E+V)logV
#[allow(dead_code)]
struct Dijkstra {
    distance: Vec<usize>,
    distance2: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut distance2 = vec![INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((Reverse(0), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            if distance2[target] < d {
                continue;
            }
            for &(next, cost) in &edge[target] {
                let mut d2 = d + cost;
                if distance[next] > d2 {
                    d2 = distance[next];
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
                }
                if distance2[next] > d2 && distance[next] < d2 {
                    distance2[next] = d2;
                    heap.push((Reverse(distance[next]), next));
                }
            }
        }
        Self {
            distance,
            distance2,
        }
    }
}
