use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        w:usize,
        h:usize,
        n:usize,
        x1:[usize;n],
        x2:[usize;n],
        y1:[usize;n],
        y2:[usize;n],
    }
    let xmap = compress(&x1, &x2, n, w);
    let ymap = compress(&y1, &y2, n, h);
    let comp_h = xmap.len();
    let comp_w = ymap.len();
    // 線を塗りつぶした部分をtrueとする
    let mut grid = vec![vec![false; comp_h]; comp_w];
    for i in 0..n {
        for y in ymap[&y1[i]]..=ymap[&y2[i]] {
            for x in xmap[&x1[i]]..=xmap[&x2[i]] {
                grid[y][x] = true;
            }
        }
    }
    // 領域を数える
    let ans = count_area(&mut grid, comp_w, comp_h, false);
    println!("{}", ans);
}

// 線を引いたマスと前後1マスのみを対象とする
fn compress(source1: &[usize], source2: &[usize], n: usize, max: usize) -> HashMap<usize, usize> {
    let mut set = HashSet::new();
    for i in 0..n {
        for d in -1..=1 {
            let t1 = source1[i] as i64 + d;
            let t2 = source2[i] as i64 + d;
            if 1 <= t1 && t1 <= max as i64 {
                set.insert(t1 as usize);
            }
            if 1 <= t2 && t2 <= max as i64 {
                set.insert(t2 as usize);
            }
        }
    }
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(x, i);
    }
    result
}

fn count_area(grid: &mut Vec<Vec<bool>>, w: usize, h: usize, target: bool) -> usize {
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    let mut ans = 0;
    for y in 0..w {
        for x in 0..h {
            if grid[y][x] != target {
                // 対象外
                continue;
            }
            ans += 1;
            let mut q = VecDeque::new();
            q.push_back((x, y));
            while let Some((sx, sy)) = q.pop_front() {
                for i in 0..4 {
                    let tx = sx as i64 + dx[i];
                    let ty = sy as i64 + dy[i];
                    if tx < 0 || w as i64 <= tx || ty < 0 || w as i64 <= ty {
                        continue;
                    }
                    let tx = tx as usize;
                    let ty = ty as usize;
                    if grid[ty][tx] != target {
                        continue;
                    }
                    q.push_back((tx, ty));
                    grid[ty][tx] = !target;
                }
            }
        }
    }
    ans
}
