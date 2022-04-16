use proconio::input;

// 分割数
fn main() {
    input! {
        n:usize,
        m:usize,
    }
    // dp[i][j] iをj分割したときの場合の数(i,jは0-index)
    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..=n {
        for j in 1..=m {
            if i < j {
                dp[i][j] = dp[i][j - 1];
            } else {
                dp[i][j] = dp[i][j - 1] + dp[i - j][j];
            }
        }
    }
    println!("{}", dp[n][m]);
}
