use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        k:usize
    }
    println!("{}", if dfs(0, 0, n, k, &a) { "Yes" } else { "No" });
}

fn dfs(idx: usize, sum: usize, n: usize, k: usize, a: &Vec<usize>) -> bool {
    if idx == n {
        return sum == k;
    }
    if dfs(idx + 1, sum, n, k, a) {
        return true;
    }
    if dfs(idx + 1, sum + a[idx], n, k, a) {
        return true;
    }
    false
}
