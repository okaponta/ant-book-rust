use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut x:[usize;n],
    }
    x.sort();
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        // 最大化する
        if is_ok(&x, med, m) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(x: &Vec<usize>, med: usize, m: usize) -> bool {
    let mut count = 1;
    let mut prev = x[0];
    for &xi in x {
        if prev + med <= xi {
            count += 1;
            prev = xi;
        }
    }
    m <= count
}
