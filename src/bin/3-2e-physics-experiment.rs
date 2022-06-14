use proconio::input;

fn main() {
    input! {
        n:usize,
        h:usize,
        r:usize,
        t:usize,
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push(calc(t - i, h));
    }
    ans.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for i in 0..n {
        println!("{:.*}", 2, ans[i] + (2 * r * i) as f64 / 100.0);
    }
}

fn calc(t: usize, h: usize) -> f64 {
    let period = ((2 * h) as f64 / 10.0).sqrt();
    let shift = ((t as f64 / period).floor() / 2.0).ceil() * 2.0;
    h as f64 - 5.0 * (t as f64 - shift * period).powi(2)
}
