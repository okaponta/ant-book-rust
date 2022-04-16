use proconio::input;

// LIS: Longest Increasing Subsequence(最長増加部分列)
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    // dp[i] 最後がaiであるような最長増加部分列の長さ
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if a[j] < a[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    let ans = dp.iter().max().unwrap_or(&0);
    println!("{}", ans);
}
