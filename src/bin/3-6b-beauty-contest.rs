use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let ps = xy.iter().map(|&(x, y)| V2::new(x, y)).collect_vec();
    let qs = convex_hull(ps, n);
    let mut ans = 0;
    for i in 0..qs.len() {
        for j in 0..i {
            ans = ans.max(dist(qs[i], qs[j]));
        }
    }
    println!("{}", ans);
}

// 凸包を求める
fn convex_hull(mut ps: Vec<V2<i64>>, n: usize) -> Vec<V2<i64>> {
    ps.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    let mut k = 0;
    let mut qs: Vec<V2<i64>> = vec![];
    // 下側凸包の作成
    for i in 0..n {
        while k > 1 && (qs[k - 1] - qs[k - 2]).det(ps[i] - qs[k - 1]) <= 0 {
            k -= 1;
        }
        if k >= qs.len() {
            qs.push(ps[i]);
        } else {
            qs[k] = ps[i];
        }
        k += 1;
    }
    // 上側凸包の作成
    let t = k;
    for i in (0..n - 1).rev() {
        while k > t && (qs[k - 1] - qs[k - 2]).det(ps[i] - qs[k - 1]) <= 0 {
            k -= 1;
        }
        if k == qs.len() {
            qs.push(ps[i]);
        } else {
            qs[k] = ps[i];
        }
        k += 1;
    }
    qs
}

fn dist(a: V2<i64>, b: V2<i64>) -> i64 {
    (a - b).dot(a - b)
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
