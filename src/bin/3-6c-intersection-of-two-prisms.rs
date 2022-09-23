use proconio::input;

fn main() {
    input! {
        m:usize,
        n:usize,
        xy:[(i64,i64);m],
        xz:[(i64,i64);n],
    }
    let mut min1 = xy[0].0;
    let mut max1 = xy[0].0;
    let mut min2 = xz[0].0;
    let mut max2 = xz[0].0;
    let mut xs = vec![];
    for i in 0..m {
        xs.push(xy[i].0);
        min1 = min1.min(xy[i].0);
        max1 = max1.max(xy[i].0);
    }
    for i in 0..n {
        xs.push(xz[i].0);
        min2 = min2.min(xz[i].0);
        max2 = max2.max(xz[i].0);
    }
    xs.sort();
    xs.dedup();

    let mut ans = 0.0;
    for i in 0..xs.len() - 1 {
        let a = xs[i];
        let b = xs[i + 1];
        if min1 * 2 <= (a + b) && (a + b) <= max1 * 2 && min2 * 2 <= (a + b) && (a + b) <= max2 * 2
        {
            let a = a as f64;
            let b = b as f64;
            let c = (a + b) / 2.0;
            // xが二次式なので、シンプソンの公式で積分値を計算
            let fa = width(m, &xy, a) * width(n, &xz, a);
            let fb = width(m, &xy, b) * width(n, &xz, b);
            let fc = width(m, &xy, c) * width(n, &xz, c);
            ans += (b - a) / 6.0 * (fa + 4.0 * fc + fb);
        }
    }
    println!("{:.*}", 4, ans);
}

fn width(n: usize, xy: &Vec<(i64, i64)>, x: f64) -> f64 {
    let mut lower: f64 = 1e10;
    let mut upper: f64 = -1e10;
    for i in 0..n {
        let x1 = xy[i].0 as f64;
        let y1 = xy[i].1 as f64;
        let x2 = xy[(i + 1) % n].0 as f64;
        let y2 = xy[(i + 1) % n].1 as f64;
        if (x1 - x) * (x2 - x) <= 0.0 && x1 != x2 {
            let y = y1 + (x - x1) * (y2 - y1) / (x2 - x1);
            lower = lower.min(y);
            upper = upper.max(y);
        }
    }
    (upper - lower).max(0.0)
}
