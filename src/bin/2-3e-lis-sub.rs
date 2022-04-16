use proconio::input;
use superslice::Ext;

const INF: usize = 1 << 60;

// LIS: Longest Increasing Subsequence(最長増加部分列)
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    // dp[i] 長さがi+1である増加部分列における最終要素の最小値
    let mut dp = vec![INF; n];
    for i in 0..n {
        let idx = dp.lower_bound(&a[i]);
        dp[idx] = a[i];
    }
    let ans = dp.lower_bound(&INF);
    println!("{}", ans);
}
