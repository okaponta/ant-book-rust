use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:[usize;n],
    }
    let mut ans = false;

    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                for d in 0..n {
                    if k[a] + k[b] + k[c] + k[d] == m {
                        ans = true;
                    }
                }
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
