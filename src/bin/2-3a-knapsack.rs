use proconio::input;

fn main() {
    input! {
        n:usize,
        wv:[(usize,usize);n],
        w:usize,
    }
    // dp[i][j] i番目までの品物から重さの総和がj以下になるように選んだ時の価値の総和の最大値
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        for j in 0..=w {
            if j < wv[i].0 {
                // 重さがj以上はそのまま
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - wv[i].0] + wv[i].1);
            }
        }
    }
    println!("{}", dp[n][w]);
}
