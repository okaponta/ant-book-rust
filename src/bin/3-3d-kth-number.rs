use num_integer::Roots;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n],
        ijk:[(Usize1,usize,usize);m],
    }
    let b = n.sqrt();
    let mut buckets = vec![vec![]; n / b + 1];
    let mut nums = vec![];
    for i in 0..n {
        buckets[i / b].push(a[i]);
        nums.push(a[i]);
    }
    nums.sort();
    for i in 0..n / b {
        buckets[i].sort();
    }

    for (i, j, k) in ijk {
        let mut lower = 0;
        let mut upper = n;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            let x = nums[med];
            let mut tl = i;
            let mut tr = j;
            let mut count = 0;
            while tl < tr && tl % b != 0 {
                if a[tl] <= x {
                    count += 1;
                }
                tl += 1;
            }
            while tl < tr && tr % b != 0 {
                tr -= 1;
                if a[tr] <= x {
                    count += 1;
                }
            }
            while tl < tr {
                let target = tl / b;
                count += buckets[target].upper_bound(&x);
                tl += b;
            }
            if count >= k {
                upper = med;
            } else {
                lower = med;
            }
        }
        println!("{}", nums[upper]);
    }
}
