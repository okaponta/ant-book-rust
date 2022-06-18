use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        n:usize,
        c:usize,
        l:[usize;n],
        s:[usize;c],
        a:[usize;c],
    }
    // 完全二分木じゃなくて、midが1になるまでのデータ構造になってる
    let mut seg = SegmentTree::new(n);
    seg.init(0, 0, n, &l);
    let mut prv = vec![PI; n];
    for i in 0..c {
        let s = s[i];
        let a = a[i] as f64 / 360.0 * 2.0 * PI;

        seg.update(s, a - prv[s], 0, 0, n);
        prv[s] = a;
        println!("{:.*} {:.*}", 2, seg.v[0].0, 2, seg.v[0].1);
    }
}

pub struct SegmentTree {
    v: Vec<(f64, f64)>,
    ang: Vec<f64>,
}

impl SegmentTree {
    pub fn new(size: usize) -> SegmentTree {
        let m = size.next_power_of_two();
        let ang = vec![0.0; 2 * m];
        let v = vec![(0.0, 0.0); 2 * m];
        SegmentTree { v, ang }
    }

    pub fn init(&mut self, k: usize, l: usize, r: usize, le: &Vec<usize>) {
        if r - l == 1 {
            self.v[k].1 = le[l] as f64;
        } else {
            let ch1 = k * 2 + 1;
            let ch2 = k * 2 + 2;
            self.init(ch1, l, (l + r) / 2, le);
            self.init(ch2, (l + r) / 2, r, le);
            self.v[k].1 = self.v[ch1].1 + self.v[ch2].1;
        }
    }

    pub fn update(&mut self, s: usize, value: f64, v: usize, l: usize, r: usize) {
        if s <= l {
            return;
        } else if s < r {
            let ch1 = v * 2 + 1;
            let ch2 = v * 2 + 2;
            let m = (l + r) / 2;
            self.update(s, value, ch1, l, m);
            self.update(s, value, ch2, m, r);
            if s <= m {
                self.ang[v] += value;
            }
            let s = self.ang[v].sin();
            let c = self.ang[v].cos();
            self.v[v] = (
                self.v[ch1].0 + (c * self.v[ch2].0 - s * self.v[ch2].1),
                self.v[ch1].1 + s * self.v[ch2].0 + c * self.v[ch2].1,
            );
        }
    }
}
