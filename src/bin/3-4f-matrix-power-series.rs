use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        m:usize,
        a:[[usize;n];n],
    }
    let mut mat = vec![vec![0; 2 * n]; 2 * n];

    let modulo = m;
    // 累積和を計算するため、以下のような行列を用意。Oはゼロ行列、Iはn*nの単位行列
    // AO
    // II
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = a[i][j];
        }
        mat[n + i][i] = 1;
        mat[n + i][n + i] = 1;
    }
    let s = pow(mat, k + 1, modulo, 2 * n);
    for i in 0..n {
        let mut ans = vec![];
        for j in 0..n {
            let mut tmp = s[n + i][j] + modulo;
            // 単位行列も累積和に含んでいるので取り除く
            if i == j {
                tmp -= 1;
            }
            ans.push(tmp % modulo);
        }
        println!("{}", ans.iter().join(" "));
    }
}

// 行列式のn乗
fn pow(mut a: Vec<Vec<usize>>, mut n: usize, modulo: usize, size: usize) -> Vec<Vec<usize>> {
    let mut b = vec![vec![0; size]; size];
    for i in 0..size {
        b[i][i] = 1;
    }
    while 0 < n {
        if n & 1 == 1 {
            b = multiply(&b, &a, size);
            rem(&mut b, modulo, size);
        }
        a = multiply(&a, &a, size);
        rem(&mut a, modulo, size);
        n >>= 1;
    }
    b
}

// 行列式の掛け算
fn multiply(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
}

// 各項のmodをとる
fn rem(a: &mut Vec<Vec<usize>>, modulo: usize, n: usize) {
    for i in 0..n {
        for j in 0..n {
            a[i][j] %= modulo;
        }
    }
}
