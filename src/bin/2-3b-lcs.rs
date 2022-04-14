use proconio::{input, marker::Chars};

// Longest Common Subsequence(最長共通部分列)
fn main() {
    input! {
        n:usize,m:usize,
        s: Chars,
        t: Chars,
    }
    // dp[i][j] s1-siまでとt1-tjまでの最長共通部分列の長さ
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    println!("{}", dp[n][m]);
}
