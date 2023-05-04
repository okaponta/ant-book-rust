use proconio::input;

fn main() {
    input! {
        n:usize,
        wvm:[(usize,usize,usize);n],
        w:usize
    }
    let mut dp = vec![0; w + 1];
    // miを1,2,4,8...,余りの重さの商品に分割してナップザック問題に帰着させる
    for (wi, vi, mut mi) in wvm {
        let mut k = 1;
        while 0 < mi {
            let mul = mi.min(k);
            for j in (wi * mul..=w).rev() {
                dp[j] = dp[j].max(dp[j - wi * mul] + vi * mul);
            }
            mi -= mul;
            k <<= 1;
        }
    }
    println!("{}", dp[w]);
}
