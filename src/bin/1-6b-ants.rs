use proconio::input;

fn main() {
    input! {
        l:usize,
        n:usize,
        x:[usize;n],
    }
    let mut min = 0;
    let mut max = 0;
    for xi in x {
        let left = xi;
        let right = l - xi;
        min = min.max(left.min(right));
        max = max.max(left.max(right));
    }
    println!("{}", min);
    println!("{}", max);
}
