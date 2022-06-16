use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        w:[usize;n],
        v:[usize;n],
        wmax:usize
    }
    let n1 = n / 2;
    let n2 = n - n1;
    // 前半部分の全列挙
    let mut first = vec![];
    for i in 0..1 << n1 {
        let mut sw = 0;
        let mut sv = 0;
        for j in 0..n1 {
            if i >> j & 1 == 1 {
                sw += w[j];
                sv += v[j];
            }
        }
        if sw <= wmax {
            first.push((sw, sv));
        }
    }

    // 辞書順にソートし、uvが単調増加になるように
    first.sort();
    let mut dict = vec![(0, 0)];
    let mut max = 0;
    for (u, v) in first {
        if v <= max {
            continue;
        }
        dict.push((u, v));
        max = v;
    }
    let mut ans = 0;
    // 後半部分の全列挙
    for i in 0..1 << n2 {
        let mut sw = 0;
        let mut sv = 0;
        for j in 0..n2 {
            if i >> j & 1 == 1 {
                sw += w[n1 + j];
                sv += v[n1 + j];
            }
        }
        if sw <= wmax {
            let pos = dict.upper_bound(&(wmax - sw, sv)) - 1;
            ans = ans.max(sv + dict[pos].1);
        }
    }
    println!("{}", ans);
}
