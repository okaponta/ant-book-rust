use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        m:usize,
        n:usize,
        a:[[usize;n];m],
    }
    let mut flipnum = 1 << 60;
    let mut ans = vec![vec![]];
    // 1列目を辞書順で試す
    for i in 0..1 << n {
        let mut flip = vec![vec![0; n]; m];
        for j in 0..n {
            flip[0][n - j - 1] = i >> j & 1;
        }
        // 2行目以降を返却
        let num = calc(&mut flip, &a, n, m);
        if 0 <= num && num < flipnum {
            ans = flip;
            flipnum = num;
        }
    }
    if flipnum == 1 << 60 {
        println!("IMPOSSIBLE");
    } else {
        for i in 0..m {
            println!("{}", ans[i].iter().join(" "));
        }
    }
}

fn calc(flip: &mut Vec<Vec<usize>>, a: &Vec<Vec<usize>>, n: usize, m: usize) -> i64 {
    for i in 1..m {
        for j in 0..n {
            // i-1, jの色を調べて上から決めていく
            let f = get(i - 1, j, flip, a, n, m);
            if f % 2 == 1 {
                flip[i][j] = 1;
            }
        }
    }
    // 最後の行チェック
    for j in 0..n {
        if get(m - 1, j, flip, a, n, m) == 1 {
            return -1;
        }
    }
    // 反転回数カウント
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res += flip[i][j];
        }
    }
    res as i64
}

fn get(
    i: usize,
    j: usize,
    flip: &Vec<Vec<usize>>,
    a: &Vec<Vec<usize>>,
    n: usize,
    m: usize,
) -> usize {
    let dx = vec![-1, 0, 0, 0, 1];
    let dy = vec![0, -1, 0, 1, 0];
    let mut f = a[i][j];
    for d in 0..5 {
        let x2 = i as i32 + dx[d];
        let y2 = j as i32 + dy[d];
        if 0 <= x2 && x2 < m as i32 && 0 <= y2 && y2 < n as i32 {
            f += flip[x2 as usize][y2 as usize];
        }
    }
    f
}
