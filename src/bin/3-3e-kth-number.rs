use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n],
        ijk:[(Usize1,usize,usize);m],
    }
    let mut nums = a.clone();
    nums.sort();

    let mut rsg = RangeSegTree::new(n);
    rsg.init(&a, 0, 0, n);

    for (i, j, k) in ijk {
        let mut lower = 0;
        let mut upper = n;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            let x = nums[med];
            let count = rsg.query(i, j, x, 0, 0, n);

            if count >= k {
                upper = med;
            } else {
                lower = med;
            }
        }
        println!("{}", nums[upper]);
    }
}

struct RangeSegTree {
    data: Vec<Vec<i64>>,
}

impl RangeSegTree {
    fn new(size: usize) -> RangeSegTree {
        let m = size.next_power_of_two();
        let data = vec![vec![]; m * 2];
        RangeSegTree { data }
    }

    fn init(&mut self, a: &Vec<i64>, k: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[k].push(a[l]);
        } else {
            let lch = k * 2 + 1;
            let rch = k * 2 + 2;
            let med = (l + r) / 2;
            self.init(a, lch, l, med);
            self.init(a, rch, med, r);
            self.data[k] = self.data[lch]
                .iter()
                .merge(self.data[rch].iter())
                .map(|i| *i)
                .collect::<Vec<i64>>();
        }
    }

    fn query(&self, i: usize, j: usize, x: i64, k: usize, l: usize, r: usize) -> usize {
        if j <= l || r <= i {
            0
        } else if i <= l && r <= j {
            self.data[k].upper_bound(&x)
        } else {
            let med = (l + r) / 2;
            let lc = self.query(i, j, x, k * 2 + 1, l, med);
            let rc = self.query(i, j, x, k * 2 + 2, med, r);
            lc + rc
        }
    }
}
