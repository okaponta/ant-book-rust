use proconio::{input, marker::Usize1};

const EPS: f64 = 1e-10;

fn main() {
    input! {
        n:usize,
        p:[(f64,f64);n],
        q:[(f64,f64);n],
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut is_connected = vec![vec![false; n]; n];
    for i in 0..n {
        is_connected[i][i] = true;
        let pi = V2::new_p(p[i]);
        let qi = V2::new_p(q[i]);
        for j in 0..i {
            let pj = V2::new_p(p[j]);
            let qj = V2::new_p(q[j]);
            if (pi - qi).det(pj - qj).abs() < EPS {
                //平行
                let is_conn = on_seg(pi, qi, pj)
                    || on_seg(pi, qi, qj)
                    || on_seg(pj, qj, pi)
                    || on_seg(pj, qj, qi);
                is_connected[i][j] = is_conn;
                is_connected[j][i] = is_conn;
            } else {
                //平行じゃない
                let r = intersection(pi, qi, pj, qj);
                let is_conn = on_seg(pi, qi, r) && on_seg(pj, qj, r);
                is_connected[i][j] = is_conn;
                is_connected[j][i] = is_conn;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                is_connected[i][j] |= is_connected[i][k] && is_connected[k][j];
            }
        }
    }
    for (a, b) in ab {
        println!("{}CONNECTED", if is_connected[a][b] { "" } else { "NOT " });
    }
}

// 直線p1-p2と直線q1-q2の交点
fn intersection(p1: V2<f64>, p2: V2<f64>, q1: V2<f64>, q2: V2<f64>) -> V2<f64> {
    p1 + (p2 - p1).mul((q2 - q1).det(q1 - p1) / (q2 - q1).det(p2 - p1))
}

// 線分p1-p2上に点qがあるかを判定
fn on_seg(p1: V2<f64>, p2: V2<f64>, q: V2<f64>) -> bool {
    (p1 - q).det(p2 - q).abs() < EPS && (p1 - q).dot(p2 - q) < EPS
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V2<T> {
    x: T,
    y: T,
}

impl<T> V2<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    pub fn new(x: T, y: T) -> V2<T> {
        V2 { x, y }
    }

    pub fn new_p(p: (T, T)) -> V2<T> {
        V2 { x: p.0, y: p.1 }
    }

    pub fn mul(&self, ope: T) -> Self {
        Self {
            x: self.x * ope,
            y: self.y * ope,
        }
    }

    pub fn dot(&self, p: V2<T>) -> T {
        self.x * p.x + self.y * p.y
    }

    pub fn det(&self, p: V2<T>) -> T {
        self.x * p.y - self.y * p.x
    }
}

impl<T> std::ops::Add for V2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Sub for V2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
