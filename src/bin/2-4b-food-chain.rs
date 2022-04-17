use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        txy:[(usize,Usize1,Usize1);k],
    }
    let mut ans = 0;
    let mut uf = UnionFind::new(3 * n);
    for (t, x, y) in txy {
        if n <= x || n <= y {
            ans += 1;
            continue;
        }
        let (xa, xb, xc) = (3 * x, 3 * x + 1, 3 * x + 2);
        let (ya, yb, yc) = (3 * y, 3 * y + 1, 3 * y + 2);
        if t == 1 {
            if uf.equiv(xa, yb) || uf.equiv(xa, yc) {
                ans += 1;
                continue;
            }
            uf.union(xa, ya);
            uf.union(xb, yb);
            uf.union(xc, yc);
        } else {
            if uf.equiv(xa, ya) || uf.equiv(xa, yc) {
                ans += 1;
                continue;
            }
            uf.union(xa, yb);
            uf.union(xb, yc);
            uf.union(xc, ya);
        }
    }
    println!("{}", ans);
}
