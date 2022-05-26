use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize
    }
    println!("{}", judge_primes(n).iter().filter(|b| **b).count());
}

fn judge_primes(n: usize) -> Vec<bool> {
    let mut res = vec![true; n + 1];
    res[0] = false;
    res[1] = false;
    for i in 2..=n.sqrt() {
        if !res[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            res[j] = false;
        }
    }
    res
}
