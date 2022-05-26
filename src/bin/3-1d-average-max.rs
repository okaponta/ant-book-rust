use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        wv:[(f64,f64);n],
    }
    let mut lower = 0.0;
    let mut upper = 1e18;
    while upper - lower > 1e-5 {
        let med = (lower + upper) / 2.0;
        // 最大化
        if is_ok(wv.clone(), med, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{:.*}", 2, lower);
}

// med以上いけるか判定する
fn is_ok(mut x: Vec<(f64, f64)>, med: f64, m: usize) -> bool {
    let key = |wv: (f64, f64)| wv.1 - med * wv.0;
    x.sort_by(|a, b| key(*b).partial_cmp(&key(*a)).unwrap());
    (0..m).map(|i| key(x[i])).sum::<f64>() >= 0.0
}
