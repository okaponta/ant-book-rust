use proconio::input;

fn main() {
    input! {
        n:usize,
        s:usize,
        a:[usize;n],
    }
    let mut ans = 1 << 60;
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    loop {
        while sum < s && right < n {
            sum += a[right];
            right += 1;
        }
        if sum < s {
            // しゃくとり終了
            break;
        }
        ans = ans.min(right - left);
        sum -= a[left];
        left += 1;
    }
    println!("{}", ans);
}
