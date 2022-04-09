use std::collections::VecDeque;

use proconio::{input, marker::Chars};

const INF: i32 = 1_000_000;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:[Chars;n],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut road = vec![vec![true; m]; n];
    for i in 0..m {
        for j in 0..n {
            if s[i][j] == '#' {
                road[i][j] = false;
            }
            if s[i][j] == 'S' {
                start = (i, j);
            }
            if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];
    let mut dist = vec![vec![INF; m]; n];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((start.0, start.1));
    dist[start.0][start.1] = 0;
    while let Some(point) = q.pop_front() {
        if point.0 == goal.0 && point.1 == goal.1 {
            break;
        }
        for i in 0..4 {
            let nx = point.0 as i32 + dx[i];
            let ny = point.1 as i32 + dy[i];
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx < n && ny < m && road[nx][ny] && dist[nx][ny] == INF {
                q.push_back((nx, ny));
                dist[nx][ny] = dist[point.0][point.1] + 1;
            }
        }
    }
    println!("{}", dist[goal.0][goal.1]);
}
