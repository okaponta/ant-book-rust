use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        xyzw:[(i64,i64,i64,i64);n],
    }
    let mut mat = vec![vec![0; n]; n];
    let mut sol = vec![0; n];
    for i in 0..n {
        mat[i][0] = xyzw[i].0;
        mat[i][1] = xyzw[i].1;
        mat[i][2] = xyzw[i].2;
        sol[i] = xyzw[i].3;
    }
    let ans = gauss_jordan(n, &mat, &sol);
    println!("{}", ans.iter().map(|f| f.round() as i64).join(" "));
}

fn gauss_jordan(n: usize, mat: &Vec<Vec<i64>>, sol: &Vec<i64>) -> Vec<f64> {
    let eps = 1e-8;
    let mut b = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = mat[i][j] as f64;
        }
        b[i][n] = sol[i] as f64;
    }

    for i in 0..n {
        let mut pivot = i;
        for j in i..n {
            if b[j][i].abs() > b[pivot][i].abs() {
                pivot = j;
            }
        }
        (b[i], b[pivot]) = (b[pivot].clone(), b[i].clone());

        if b[i][i].abs() < eps {
            return vec![];
        }

        for j in i + 1..=n {
            b[i][j] /= b[i][i];
        }
        for j in 0..n {
            if i != j {
                for k in i + 1..=n {
                    b[j][k] -= b[j][i] * b[i][k];
                }
            }
        }
    }
    let mut res = vec![];
    for i in 0..n {
        res.push(b[i][n]);
    }
    res
}
