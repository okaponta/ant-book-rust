use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", judge_seg_primes(a, b).iter().filter(|b| **b).count());
}

fn judge_seg_primes(a: usize, b: usize) -> Vec<bool> {
    let sq = b.sqrt();
    let mut res = vec![true; b - a];
    let mut small = vec![true; sq + 1];
    small[0] = false;
    small[1] = false;
    for i in 2..=sq {
        if !small[i] {
            continue;
        }
        for j in (i * i..=sq).step_by(i) {
            small[j] = false;
        }
        // aをこえるiの最小の倍数
        let a_min = ((a + i - 1) / i) * i;
        for j in (a_min..b).step_by(i) {
            res[j - a] = false;
        }
    }
    res
}
