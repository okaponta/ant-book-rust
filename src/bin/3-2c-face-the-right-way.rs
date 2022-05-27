use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut is_back = vec![0; n];
    for i in 0..n {
        if s[i] == 'B' {
            is_back[i] = 1;
        }
    }
    let mut ans = n;
    let mut ansk = 0;
    // kは牛の数
    for k in 1..n {
        let mut count = 0;
        // 区間でひっくりかえした総数を保持
        let mut sum = 0;
        // ひっくり返すときはflip[i]=1とする
        let mut flip = vec![0; n];
        for i in 0..=(n - k) {
            if (sum + is_back[i]) % 2 == 1 {
                // 後ろ向きなのでひっくり返す必要あり
                flip[i] = 1;
                count += 1;
                sum += 1;
            }
            if k <= i + 1 {
                // 左の区間で減った分を考慮
                sum -= flip[i + 1 - k];
            }
        }
        let mut is_ok = true;
        // 残った牛の整合性チェック
        for i in n + 1 - k..n {
            if (sum + is_back[i]) % 2 == 1 {
                is_ok = false;
                break;
            }
            sum -= flip[i + 1 - k];
        }
        if is_ok && count < ans {
            ans = count;
            ansk = k;
        }
    }
    println!("{}", ans);
    println!("{}", ansk);
}
