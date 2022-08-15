use std::ops::Mul;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:Usize1,
    }
    let modulo = 1000;
    // 行列
    let a = nalgebra::Matrix2::new(1, 1, 1, 0);
    let a = pow(a, n, modulo);
    println!("{}", a[(1, 0)]);
}

// 行列式のn乗
fn pow(mut a: nalgebra::Matrix2<usize>, mut n: usize, modulo: usize) -> nalgebra::Matrix2<usize> {
    let mut b = a.clone();
    while 0 < n {
        if n & 1 == 1 {
            b = b.mul(a);
            rem(&mut b, modulo);
        }
        a = a.mul(a);
        rem(&mut a, modulo);
        n >>= 1;
    }
    b
}

// 各項のmodをとる
fn rem(a: &mut nalgebra::Matrix2<usize>, modulo: usize) {
    a.m11 %= modulo;
    a.m12 %= modulo;
    a.m21 %= modulo;
    a.m22 %= modulo;
}
