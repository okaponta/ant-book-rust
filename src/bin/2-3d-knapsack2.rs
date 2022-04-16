use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n:usize,
        wv:[(usize,usize);n],
        w:usize,
    }
    // dp[i][j] i番目までの品物から「価値」の総和がj以下になるように選んだ時の重さの総和の最小値
    // 問題によってはdpを使い回すことでメモリ節約が可能
    let mut dp = vec![vec![INF; 100 * n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=100 * n {
            if j < wv[i].1 {
                // 対象商品の価値がj以上の場合は選べない
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i][j].min(dp[i][j - wv[i].1] + wv[i].0);
            }
        }
    }
    let ans = dp[n]
        .iter()
        .enumerate()
        .filter(|(_, wi)| wi <= &&w)
        .map(|(i, _)| i)
        .max()
        .unwrap_or(0);
    println!("{}", ans);
}
