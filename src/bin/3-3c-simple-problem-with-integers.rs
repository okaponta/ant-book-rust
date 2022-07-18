use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[i64;n],
    }
    let mut seg = LazySegmentTree::new(n);
    for i in 0..n {
        seg.add(i, i + 1, a[i]);
    }
    for _ in 0..q {
        input! {
            q:char, l:Usize1, r:usize,
        }
        if q == 'C' {
            input! {x:i64}
            seg.add(l, r, x);
        } else {
            println!("{}", seg.sum(l, r));
        }
    }
}

struct LazySegmentTree {
    n: usize,
    node: Vec<i64>,
    lazy: Vec<i64>,
}

#[allow(dead_code)]
impl LazySegmentTree {
    fn new(size: usize) -> LazySegmentTree {
        let n = size.next_power_of_two();
        LazySegmentTree {
            n: n,
            node: vec![0i64; 2 * n],
            lazy: vec![0i64; 2 * n],
        }
    }

    // k番目のノードの遅延評価
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.node[k] += self.lazy[k];
        }
        if r - l > 1 {
            // 最下段かどうかのチェック
            self.lazy[2 * k + 1] += self.lazy[k] / 2;
            self.lazy[2 * k + 2] += self.lazy[k] / 2;
        }
        self.lazy[k] = 0;
    }

    // [a,b)にxを加算する
    fn add(&mut self, a: usize, b: usize, x: i64) {
        self.add_range(a, b, x, 0, 0, self.n)
    }

    // [a,b)の合計値
    fn sum(&mut self, a: usize, b: usize) -> i64 {
        self.sum_range(a, b, 0, 0, self.n)
    }

    fn add_range(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] += (r - l) as i64 * x;
            self.eval(k, l, r);
        } else {
            self.add_range(a, b, x, k * 2 + 1, l, (l + r) / 2);
            self.add_range(a, b, x, k * 2 + 2, (l + r) / 2, r);
            self.node[k] = self.node[2 * k + 1] + self.node[2 * k + 2];
        }
    }

    fn sum_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return 0;
        }
        self.eval(k, l, r);
        if a <= l && r <= b {
            return self.node[k] as i64;
        }
        let left = self.sum_range(a, b, 2 * k + 1, l, (l + r) / 2);
        let right = self.sum_range(a, b, 2 * k + 2, (l + r) / 2, r);
        return left + right;
    }
}

struct LazySegmentTree2 {
    n: usize,
    node: Vec<i64>,
    lazy: Vec<i64>,
}

#[allow(dead_code)]
impl LazySegmentTree2 {
    fn new(size: usize) -> LazySegmentTree2 {
        let n = size.next_power_of_two();
        LazySegmentTree2 {
            n: n,
            node: vec![0i64; 2 * n],
            lazy: vec![0i64; 2 * n],
        }
    }

    // k番目のノードの遅延評価
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.node[k] += self.lazy[k];
        }
        if r - l > 1 {
            // 最下段かどうかのチェック
            self.lazy[2 * k + 1] += self.lazy[k] / 2;
            self.lazy[2 * k + 2] += self.lazy[k] / 2;
        }
        self.lazy[k] = 0;
    }

    // [a,b)にxを加算する
    fn add(&mut self, a: usize, b: usize, x: i64) {
        self.add_range(a, b, x, 0, 0, self.n)
    }

    // [a,b)の合計値
    fn sum(&mut self, a: usize, b: usize) -> i64 {
        self.sum_range(a, b, 0, 0, self.n)
    }

    fn add_range(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.node[k] += x;
        } else if l < b && a < r {
            self.lazy[k] += (b.min(r) - a.max(l)) as i64 * x;
            self.add_range(a, b, x, k * 2 + 1, l, (l + r) / 2);
            self.add_range(a, b, x, k * 2 + 2, (l + r) / 2, r);
        }
    }

    fn sum_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return 0;
        }
        if a <= l && r <= b {
            return self.node[k] * (r - l) as i64 + self.lazy[k];
        }
        let mut res = (b.min(r) - a.max(l)) as i64 * self.node[k];
        res += self.sum_range(a, b, 2 * k + 1, l, (l + r) / 2);
        res += self.sum_range(a, b, 2 * k + 2, (l + r) / 2, r);
        return res;
    }
}
