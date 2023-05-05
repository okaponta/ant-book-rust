use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }
    let st = s.iter().collect::<String>();
    let agct = vec!['A', 'G', 'C', 'T'];
    // 1文字加えた際に移動する先の状態
    let mut next = vec![vec![0; 4]; k];
    for i in 0..k {
        for j in 0..4 {
            let mut str = (0..i).into_iter().map(|i| s[i]).collect::<String>();
            str.push(agct[j]);
            while &st[0..str.len()] != &str {
                str.remove(0);
            }
            next[i][j] = str.len();
        }
    }

    // dp[i][j] i番目の数字で、禁止文字列の先頭j文字一致している(長く一致している方が優先)
    let mut dp = vec![vec![0; k]; n + 1];
    dp[0][0] = 1;
    for t in 0..n {
        for i in 0..k {
            for j in 0..4 {
                let ti = next[i][j];
                if ti == k {
                    // Sと一致するため、ダメ。
                    continue;
                }
                dp[t + 1][ti] = (dp[t + 1][ti] + dp[t][i]) % 10009;
            }
        }
    }

    let mut ans = 0;
    for i in 0..k {
        ans += dp[n][i];
        ans %= 10009;
    }
    println!("{}", ans);
}
