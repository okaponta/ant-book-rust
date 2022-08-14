use std::vec;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        uvc:[(usize,usize,usize);m],
    }
    let inf = 1 << 60;
    let mut edges = vec![vec![]; n];
    for (u, v, c) in uvc {
        edges[u].push((v, c));
    }
    // dp[S][i]
    // Sがこれまで通ってきた点の集合(右が0、左がnのbit)
    // iが直前にいたマス
    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[0][0] = 0;
    for i in 0..1 << n {
        for j in 0..n {
            if dp[i][j] == inf {
                // 到達不可
                continue;
            }
            for &(next, cost) in &edges[j] {
                if i >> next & 1 == 1 {
                    // 訪問済み
                    continue;
                }
                dp[i | 1 << next][next] = dp[i | 1 << next][next].min(dp[i][j] + cost);
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
}
