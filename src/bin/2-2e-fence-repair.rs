use proconio::input;

fn main() {
    input! {
        mut n:usize,
        mut l:[usize;n],
    }
    let mut ans = 0;
    while n > 1 {
        let mut mii1 = 0;
        let mut mii2 = 1;
        for i in 2..n {
            if l[i] < l[mii1] {
                mii2 = mii1;
                mii1 = i;
            } else if l[i] < l[mii2] {
                mii2 = i;
            }
        }
        let t = l[mii1] + l[mii2];
        ans += t;

        if mii1 == n - 1 {
            // nが減少するので、最小の数字は前のインデックスにもってくる
            mii1 = mii2;
            mii2 = n - 1;
        }
        l[mii1] = t;
        l[mii2] = l[n - 1]; // 走査対象外になったので
        n -= 1;
    }
    println!("{}", ans);
}
