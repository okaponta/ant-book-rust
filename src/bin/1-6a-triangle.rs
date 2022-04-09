use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if a[i] + a[j] > a[k] && a[j] + a[k] > a[i] && a[k] + a[i] > a[j] {
                    ans = ans.max(a[i] + a[j] + a[k]);
                }
            }
        }
    }
    println!("{}", ans);
}
