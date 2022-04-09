use proconio::input;

fn main() {
    input! {
        c:[i32;6],
        mut a:i32,
    }
    let val = vec![1, 5, 10, 50, 100, 500];
    let mut ans = 0;
    for i in (0..6).rev() {
        let t = (a / val[i]).min(c[i]);
        a -= t * val[i];
        ans += t;
    }
    println!("{}", ans);
}
