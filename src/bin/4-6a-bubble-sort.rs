use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    println!("{}", merge_count(&mut a));
}

fn merge_count(a: &mut Vec<usize>) -> usize {
    let n = a.len();
    if n <= 1 {
        return 0;
    }

    let mut count = 0;
    let mut b = (0..n / 2).into_iter().map(|i| a[i]).collect::<Vec<_>>();
    let mut c = (n / 2..n).into_iter().map(|i| a[i]).collect::<Vec<_>>();
    count += merge_count(&mut b);
    count += merge_count(&mut c);

    let mut ai = 0;
    let mut bi = 0;
    let mut ci = 0;
    // マージソート
    while ai < n {
        if bi < b.len() && (ci == c.len() || b[bi] <= c[ci]) {
            a[ai] = b[bi];
            bi += 1;
        } else {
            count += n / 2 - bi;
            a[ai] = c[ci];
            ci += 1;
        }
        ai += 1;
    }
    count
}
