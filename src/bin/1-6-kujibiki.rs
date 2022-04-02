use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:[usize;n],
    }
    let mut ans = false;

    let mut kk = vec![0; n * n];
    for c in 0..n {
        for d in 0..n {
            kk[c * n + d] = k[c] + k[d];
        }
    }

    for a in 0..n {
        for b in 0..n {
            if kk.binary_search(&(m - k[a] - k[b])).is_ok() {
                ans = true;
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
