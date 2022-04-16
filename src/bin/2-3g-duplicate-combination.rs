use proconio::input;

// 分割数
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    // dp[i][j] i番目まででj個選ぶ組み合わせ
    let mut dp = vec![vec![0; m + 1]; n + 1];
    // 1つも選ばない場合
    for i in 0..=n {
        dp[i][0] = 1;
    }
    for i in 0..n {
        for j in 1..=m {
            if j - 1 >= a[i] {
                dp[i + 1][j] = dp[i + 1][j - 1] + dp[i][j] - dp[i][j - 1 - a[i]];
            } else {
                dp[i + 1][j] = dp[i + 1][j - 1] + dp[i][j];
            }
        }
    }
    println!("{}", dp[n][m]);
}
