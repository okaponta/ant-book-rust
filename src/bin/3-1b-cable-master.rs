use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        l:[f64;n],
    }
    let mut lower = 0.0;
    let mut upper = 1e18;
    while upper - lower > 1e-5 {
        let med = (lower + upper) / 2.0;
        if can_longer(&l, k, med) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{:.*}", 2, (upper * 100.0).floor() / 100.0);
}

fn can_longer(l: &Vec<f64>, k: usize, med: f64) -> bool {
    let s = l.iter().map(|f| (f / med) as usize).sum::<usize>();
    k <= s
}
