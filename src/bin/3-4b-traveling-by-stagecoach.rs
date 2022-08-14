use std::vec;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:Usize1,
        b:Usize1,
        t:[usize;n],
        uvc:[(Usize1,Usize1,usize);m],
    }
    let inf = 1e10;
    let mut edges = vec![vec![]; m];
    for (u, v, c) in uvc {
        edges[u].push((v, c));
        edges[v].push((u, c));
    }
    // dp[S][i]
    // Sがこれまで使ったチケットの集合(右が0、左がnのbit)
    // iが今いる都市
    let mut dp = vec![vec![inf; m]; 1 << n];
    dp[0][a] = 0.0;
    for i in 0..1 << n {
        for j in 0..m {
            for &(next, cost) in &edges[j] {
                // kが使うチケット
                for k in 0..n {
                    // チケット使用済
                    if i >> k & 1 == 1 {
                        continue;
                    }
                    dp[i | 1 << k][next] =
                        min(dp[i | 1 << k][next], dp[i][j] + (cost as f64 / t[k] as f64));
                }
            }
        }
    }
    let mut ans = inf;
    for i in 0..1 << n {
        ans = min(ans, dp[i][b]);
    }
    if ans == inf {
        println!("Impossible");
        return;
    }
    println!("{:.*}", 3, ans);
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
