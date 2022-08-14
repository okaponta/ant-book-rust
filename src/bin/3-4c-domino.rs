use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Chars;n],
    }
    // dp[i][j][S]
    // i行j列まで埋めて、境界がSとなるパターン数
    // ■■■■■
    // ■■★□■
    // ■□□□□
    // はdp[1][2][10001]
    // (i,j)は埋まったり黒なら飛ばす
    // 埋まってなければ縦か横で置く。
    // メモリ節約のときは現在と直前だけもってればOK。
    let mut dp = vec![vec![vec![0; 1 << m]; m + 1]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..m {
            for k in 0..1 << m {
                if k >> j & 1 == 1 || a[i][j] == 'x' {
                    // (i,j)が空じゃない
                    let next = k & !(1 << j);
                    if j + 1 < m {
                        dp[i][j + 1][next] += dp[i][j][k];
                    } else {
                        //改行
                        dp[i + 1][0][next] += dp[i][j][k];
                    }
                } else {
                    // (i,j)が空
                    if j + 1 < m && a[i][j + 1] == '.' && (k >> (j + 1)) & 1 == 0 {
                        // 横置き
                        let next = k | 1 << (j + 1);
                        dp[i][j + 1][next] += dp[i][j][k];
                    }
                    if i + 1 < n && a[i + 1][j] == '.' {
                        // 縦置き
                        let next = k | (1 << j);
                        if j + 1 < m {
                            dp[i][j + 1][next] += dp[i][j][k];
                        } else {
                            // 改行
                            dp[i + 1][0][next] += dp[i][j][k];
                        }
                    }
                }
            }
        }
    }
    println!("{}", dp[n][0][0]);
}
