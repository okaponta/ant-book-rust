use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let modulo = 10007;
    // フィボナッチの漸化式を行列で表現
    let a = nalgebra::Matrix3::new(2, 1, 0, 2, 2, 2, 0, 1, 2);
    let a = pow(a, n, modulo);
    println!("{}", a.index((0, 0)));
}

// 行列式のn乗
fn pow(mut a: nalgebra::Matrix3<usize>, mut n: usize, modulo: usize) -> nalgebra::Matrix3<usize> {
    let mut b = nalgebra::Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1);
    while 0 < n {
        if n & 1 == 1 {
            b = b * a;
            rem(&mut b, modulo);
        }
        a = a * a;
        rem(&mut a, modulo);
        n >>= 1;
    }
    b
}

// 各項のmodをとる
fn rem(a: &mut nalgebra::Matrix3<usize>, modulo: usize) {
    a.m11 %= modulo;
    a.m12 %= modulo;
    a.m13 %= modulo;
    a.m21 %= modulo;
    a.m22 %= modulo;
    a.m23 %= modulo;
    a.m31 %= modulo;
    a.m32 %= modulo;
    a.m33 %= modulo;
}
