use proconio::input;

fn main() {
    input! {
        n:usize,
        r:usize,
        mut x:[usize;n],
    }
    x.sort();
    let mut i = 0;
    let mut ans = 0;
    while i < n {
        let s = x[i];
        while i < n && x[i] <= s + r {
            i += 1;
        }
        let p = x[i - 1];
        while i < n && x[i] <= p + r {
            i += 1;
        }
        ans += 1;
    }
    println!("{}", ans);
}
